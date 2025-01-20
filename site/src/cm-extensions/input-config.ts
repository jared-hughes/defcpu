import {
  EditorView,
  ViewUpdate,
  Command,
  PanelConstructor,
  showPanel,
  Panel,
  getPanel,
} from "@codemirror/view";
import {
  EditorState,
  StateField,
  StateEffect,
  Extension,
} from "@codemirror/state";
import elt from "crelt";

export function inputConfigPanel(): Extension {
  return [inputConfigPanelState, inputConfigField, baseTheme];
}

export interface InputConfigJSON {
  arg0: string;
  argv: string;
  envp: string;
  randomSeed: string;
  rand16: string;
  vdsoPtr: string;
  execfnPtr: string;
  platformOffset: string;
  useFixed: boolean;
}

// TODO-cm: make them all undefined by default, so we know if the user changed it.
export class InputConfig {
  readonly arg0: string;
  readonly argv: string;
  readonly envp: string;
  readonly randomSeed: string;
  readonly rand16: string;
  readonly vdsoPtr: string;
  readonly execfnPtr: string;
  readonly platformOffset: string;
  readonly useFixed: boolean;

  constructor(config: InputConfigJSON) {
    this.arg0 = config.arg0;
    this.argv = config.argv;
    this.envp = config.envp;
    this.randomSeed = config.randomSeed;
    this.rand16 = config.rand16;
    this.vdsoPtr = config.vdsoPtr;
    this.execfnPtr = config.execfnPtr;
    this.platformOffset = config.platformOffset;
    this.useFixed = config.useFixed;
  }

  eq(other: InputConfig) {
    return (
      this.arg0 === other.arg0 &&
      this.argv === other.argv &&
      this.envp === other.envp &&
      this.randomSeed === other.randomSeed &&
      this.rand16 === other.rand16 &&
      this.vdsoPtr === other.vdsoPtr &&
      this.execfnPtr === other.execfnPtr &&
      this.platformOffset === other.platformOffset &&
      this.useFixed === other.useFixed
    );
  }

  static default = new InputConfig({
    arg0: "/tmp/asm",
    argv: JSON.stringify([
      "__46____1",
      "_8__4367_",
      "____2____",
      "__5______",
      "8__1_47_2",
      "__7_68__5",
      "97_31_2_4",
      "416_8__9_",
      "_52__91__",
    ]),
    envp: "[]",
    randomSeed: "123456",
    rand16: "0123456789abcdef0a2b2c3d4e5f6789",
    vdsoPtr: "0x00007FFC29BBD000",
    execfnPtr: "0x00007FFC29BAFFEF",
    platformOffset: "0x0000000000001086",
    useFixed: false,
  });

  static fromJSON(obj: unknown) {
    const defaultConfig = InputConfig.default;
    if (typeof obj !== "object" || obj === null) return defaultConfig;
    const c = {
      arg0: getString(obj, "arg0", defaultConfig.arg0),
      argv: getString(obj, "argv", defaultConfig.argv),
      envp: getString(obj, "envp", defaultConfig.envp),
      randomSeed: getString(obj, "randomSeed", defaultConfig.randomSeed),
      rand16: getString(obj, "rand16", defaultConfig.rand16),
      vdsoPtr: getString(obj, "vdsoPtr", defaultConfig.vdsoPtr),
      execfnPtr: getString(obj, "execfnPtr", defaultConfig.execfnPtr),
      platformOffset: getString(
        obj,
        "platformOffset",
        defaultConfig.platformOffset
      ),
      useFixed: ("useFixed" in obj && !!obj.useFixed) ?? defaultConfig.useFixed,
    };
    return new InputConfig(c);
  }

  /** Return a JSON-serializable object. */
  toJSON() {
    return {
      arg0: this.arg0,
      argv: this.argv,
      envp: this.envp,
      randomSeed: this.randomSeed,
      rand16: this.rand16,
      vdsoPtr: this.vdsoPtr,
      execfnPtr: this.execfnPtr,
      platformOffset: this.platformOffset,
      useFixed: this.useFixed,
    };
  }
}

function getString(obj: unknown, key: string, defaultValue: string) {
  if (typeof obj !== "object" || obj === null) return defaultValue;
  const val = (obj as any)[key];
  if (typeof val !== "string") return defaultValue;
  return val || defaultValue;
}

export const setInputConfig = StateEffect.define<InputConfig>();

export const inputConfigField = StateField.define<InputConfig>({
  create() {
    return InputConfig.default;
  },
  update(set, transaction) {
    for (const e of transaction.effects) {
      if (e.is(setInputConfig)) {
        set = e.value;
      }
    }
    return set;
  },
  toJSON(value) {
    return value.toJSON();
  },
  fromJSON(obj) {
    return InputConfig.fromJSON(obj);
  },
});

export function getInputConfig(state: EditorState) {
  const inputConfig = state.field(inputConfigField, false);
  return inputConfig ? inputConfig : InputConfig.default;
}

function phrase(view: EditorView, phrase: string) {
  return view.state.phrase(phrase);
}

// based on @codemirror/search panel.
class InputConfigPanel implements Panel {
  arg0Field: HTMLInputElement;
  argvField: HTMLInputElement;
  envpField: HTMLInputElement;
  randomSeedField: HTMLInputElement;
  rand16Field: HTMLInputElement;
  vdsoPtrField: HTMLInputElement;
  execfnPtrField: HTMLInputElement;
  platformOffsetField: HTMLInputElement;
  useFixedField: HTMLInputElement;
  dom: HTMLElement;
  inputConfig: InputConfig;

  constructor(readonly view: EditorView) {
    const inputConfig = view.state.field(inputConfigField);
    this.inputConfig = inputConfig;
    const commit = this.commit.bind(this);

    function stringField(
      name: string,
      placeholder: string,
      attrs?: { [attr: string]: any }
    ) {
      return elt("input", {
        value: "",
        placeholder,
        class: "cm-textfield",
        name: name,
        form: "",
        onchange: commit,
        onkeyup: commit,
        ...attrs,
      }) as HTMLInputElement;
    }

    const d = InputConfig.default;

    // TODO-cm: placeholders for stuff like rand16 should
    // be computed based on the random seed.
    this.arg0Field = stringField("argv", d.arg0);
    this.argvField = stringField("argv", d.argv, {
      "main-field": "true",
    });
    this.envpField = stringField("envp", d.envp);
    this.randomSeedField = stringField("randomSeed", d.randomSeed);
    this.rand16Field = stringField("rand16", d.rand16);
    this.vdsoPtrField = stringField("vdsoPtr", d.vdsoPtr);
    this.execfnPtrField = stringField("execfnPtr", d.execfnPtr);
    this.platformOffsetField = stringField("platformOffset", d.platformOffset);
    this.useFixedField = elt("input", {
      type: "checkbox",
      name: "case",
      form: "",
      checked: inputConfig.useFixed,
      onchange: commit,
    }) as HTMLInputElement;

    this.dom = elt("div", { class: "cm-input-config" }, [
      elt("label", null, [phrase(view, "arg0"), this.arg0Field]),
      elt("label", null, [phrase(view, "argv"), this.argvField]),
      elt("label", null, [phrase(view, "envp"), this.envpField]),
      elt("label", null, [phrase(view, "randomSeed"), this.randomSeedField]),
      elt("label", null, [phrase(view, "rand16"), this.rand16Field]),
      elt("label", null, [phrase(view, "vdsoPtr"), this.vdsoPtrField]),
      elt("label", null, [phrase(view, "execfnPtr"), this.execfnPtrField]),
      elt("label", null, [
        phrase(view, "platformOffset"),
        this.platformOffsetField,
      ]),
      elt("label", null, [phrase(view, "use fixed"), this.useFixedField]),
      elt(
        "button",
        {
          name: "close",
          onclick: () => closeInputConfigPanel(view),
          "aria-label": phrase(view, "close"),
          type: "button",
        },
        ["×"]
      ),
    ]);
    this.setInputConfig(inputConfig);
  }

  commit() {
    const inputConfig = new InputConfig({
      arg0: this.arg0Field.value,
      argv: this.argvField.value,
      envp: this.envpField.value,
      randomSeed: this.randomSeedField.value,
      rand16: this.rand16Field.value,
      vdsoPtr: this.vdsoPtrField.value,
      execfnPtr: this.execfnPtrField.value,
      platformOffset: this.platformOffsetField.value,
      useFixed: this.useFixedField.checked,
    });
    if (!inputConfig.eq(this.inputConfig)) {
      this.inputConfig = inputConfig;
      this.setDisabled(inputConfig);
      this.view.dispatch({ effects: setInputConfig.of(inputConfig) });
    }
  }

  update(update: ViewUpdate) {
    for (let tr of update.transactions)
      for (let effect of tr.effects) {
        if (effect.is(setInputConfig) && !effect.value.eq(this.inputConfig))
          this.setInputConfig(effect.value);
      }
  }

  /** Sync all parts of the DOM that haven't already been changed by the user. */
  setDisabled(inputConfig: InputConfig) {
    // TODO-cm: readonly while running?
    this.randomSeedField.disabled = !!inputConfig.useFixed;
    this.rand16Field.disabled = !inputConfig.useFixed;
    this.vdsoPtrField.disabled = !inputConfig.useFixed;
    this.execfnPtrField.disabled = !inputConfig.useFixed;
    this.platformOffsetField.disabled = !inputConfig.useFixed;
  }

  /** Sync DOM exactly. */
  setInputConfig(inputConfig: InputConfig) {
    this.inputConfig = inputConfig;
    this.arg0Field.value = inputConfig.arg0;
    this.argvField.value = inputConfig.argv;
    this.envpField.value = inputConfig.envp;
    this.randomSeedField.value = inputConfig.randomSeed;
    this.rand16Field.value = inputConfig.rand16;
    this.vdsoPtrField.value = inputConfig.vdsoPtr;
    this.execfnPtrField.value = inputConfig.execfnPtr;
    this.platformOffsetField.value = inputConfig.platformOffset;
    this.useFixedField.checked = inputConfig.useFixed;
    this.setDisabled(inputConfig);
  }

  mount() {
    this.argvField.select();
  }

  get pos() {
    return 80;
  }

  get top() {
    return false;
  }
}

function createInputConfigPanel(view: EditorView) {
  return new InputConfigPanel(view);
}

class InputConfigPanelState {
  constructor(readonly panel: PanelConstructor | null) {}
}

const toggleInputConfigPanel = StateEffect.define<boolean>();

const inputConfigPanelState: StateField<InputConfigPanelState> =
  StateField.define<InputConfigPanelState>({
    create() {
      return new InputConfigPanelState(null);
    },
    update(value, tr) {
      for (let effect of tr.effects) {
        if (effect.is(toggleInputConfigPanel))
          value = new InputConfigPanelState(
            effect.value ? createInputConfigPanel : null
          );
      }
      return value;
    },
    provide: (f) => showPanel.from(f, (val) => val.panel),
  });

/// Close the input config panel.
export const closeInputConfigPanel: Command = (view) => {
  let state = view.state.field(inputConfigPanelState, false);
  if (!state || !state.panel) return false;
  let panel = getPanel(view, createInputConfigPanel);
  if (panel && panel.dom.contains(view.root.activeElement)) view.focus();
  view.dispatch({ effects: toggleInputConfigPanel.of(false) });
  return true;
};

function getMainField(view: EditorView) {
  let panel = getPanel(view, createInputConfigPanel);
  return (
    panel &&
    (panel.dom.querySelector("[main-field]") as HTMLInputElement | null)
  );
}

/// Make sure the input config panel is open and focused.
export const openInputConfigPanel: Command = (view) => {
  let panelState = view.state.field(inputConfigPanelState, false);
  if (panelState && panelState.panel) {
    let mainField = getMainField(view);
    if (mainField && mainField != view.root.activeElement) {
      mainField.focus();
      mainField.select();
    }
  } else {
    const effects = [];
    effects.push(toggleInputConfigPanel.of(true));
    view.dispatch({ effects });
  }
  return true;
};

const baseTheme = EditorView.baseTheme({
  ".cm-panel.cm-input-config": {
    padding: "2px 6px 4px",
    position: "relative",
    "& [name=close]": {
      position: "absolute",
      top: "0",
      right: "4px",
      backgroundColor: "inherit",
      border: "none",
      fontSize: "200%",
      padding: 0,
      margin: 0,
      cursor: "pointer",
    },
    "& input": {
      margin: ".2em .8em .2em .4em",
    },
    "& input[type=checkbox]": {
      marginRight: ".2em",
    },
    "& label": {
      whiteSpace: "pre",
    },
  },
});

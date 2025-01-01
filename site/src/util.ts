/** Assume $ always succeeds and returns an HTMLElement */
export function $<MatchType extends HTMLElement>(selector: string) {
  return document.querySelector(selector) as MatchType;
}

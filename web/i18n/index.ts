import zh from "./zh.json";
import en from "./en.json";

export type I18n = typeof zh;

const data = (navigator.language === "zh-CN" ? zh : en) as I18n;

export function i18n(label: keyof I18n, params?: string[]) {
  const val = data[label];

  if (params === undefined) return val;
  return val.replace(/{(\d)}/g, (_, p) => params[parseInt(p)]);
}

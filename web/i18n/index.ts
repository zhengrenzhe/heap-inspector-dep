import zh from "./zh.json";
import en from "./en.json";

const data = (navigator.language === "zh-CN" ? zh : en) as typeof zh;

export function i18n(label: keyof typeof zh, params?: string[]) {
  const val = data[label];

  if (params === undefined) return val;
  return val.replace(/{(\d)}/g, (_, p) => params[parseInt(p)]);
}

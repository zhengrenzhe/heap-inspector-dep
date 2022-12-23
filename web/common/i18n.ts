const config = {
  loading: {
    "zh-CN": "加载中",
    en: "loading",
  },
};
const lang = navigator.language as "zh-CN" | "en";

export function __(key: keyof typeof config) {
  return config[key][lang];
}

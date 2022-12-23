const config = {
  loading: {
    "zh-CN": "加载中",
    en: "loading",
  },
  filter: {
    "zh-CN": "过滤",
    en: "Filter",
  },
  repeated_string: {
    "zh-CN": "重复字符串",
    en: "Repeated String",
  },
  source: {
    "zh-CN": "来源",
    en: "Source",
  },
  "source-desc": {
    "zh-CN": "在堆的指定位置中搜索包含关键字的节点",
    en: "Searches the heap for a node containing the key at the specified location",
  },
  constructor_name: {
    "zh-CN": "构造器名称",
    en: "constructor_name",
  },
  closure_name: {
    "zh-CN": "闭包名称",
    en: "closure_name",
  },
  string_value: {
    "zh-CN": "字符串值",
    en: "string_value",
  },
};
const lang = navigator.language as "zh-CN" | "en";

export function __(key: keyof typeof config) {
  return config[key][lang];
}

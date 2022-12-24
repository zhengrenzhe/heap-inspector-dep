const config = {
  loading: {
    "zh-CN": "加载中",
    en: "loading",
  },
  search: {
    "zh-CN": "搜索",
    en: "Search",
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
  self_size: {
    "zh-CN": "自身尺寸",
    en: "Self Size",
  },
  self_size_desc: {
    "zh-CN": "节点自身大小满足条件才出现在结果中，单位为 bytes",
    en: "Only when the size of the node itself meets the conditions will it appear in the result, the unit is bytes",
  },
  less_than: {
    "zh-CN": "小于",
    en: "Less Than",
  },
  more_than: {
    "zh-CN": "大于",
    en: "More Than",
  },
  retained_size: {
    "zh-CN": "持有尺寸",
    en: "Retained Size",
  },
  retained_size_desc: {
    "zh-CN": "节点持有的对象大小满足条件才出现在结果中，单位为 bytes",
    en: "The size of the object held by the node will only appear in the result if the condition is met, and the unit is bytes",
  },
  depth: {
    "zh-CN": "引用深度",
    en: "Depth",
  },
  depth_desc: {
    "zh-CN": "不满足引用深度条件的对象将不会显示",
    en: "Objects that do not meet the reference depth criteria will not be displayed",
  },
  apply_filter: {
    "zh-CN": "应用过滤条件",
    en: "Apply Filter",
  },
  dark_mode: {
    "zh-CN": "深色模式",
    en: "Dark Mode",
  },
  light_mode: {
    "zh-CN": "浅色模式",
    en: "Light Mode",
  },
  node_count: {
    "zh-CN": "节点数量",
    en: "Node Count",
  },
  edge_count: {
    "zh-CN": "边数量",
    en: "Edge Count",
  },
  file_size: {
    "zh-CN": "文件大小",
    en: "File Size",
  },
};
const lang = navigator.language as "zh-CN" | "en";

export function __(key: keyof typeof config) {
  return config[key][lang];
}

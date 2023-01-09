// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
const isDev: boolean = (import.meta as unknown).env["DEV"];

const base = isDev ? `http://${location.hostname}:${9999}` : location.href;

const createURL = (url: string) => new URL(url, base).toString();

export const API = {
  is_ready: createURL("/api/is_ready"),
  search: createURL("/api/search"),
  meta: createURL("/api/meta"),
  statistics: createURL("/api/statistics"),
};

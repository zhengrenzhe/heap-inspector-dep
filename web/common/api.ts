const port = new URL(location.href).searchParams.get("port");

const base = port ? `http://${location.hostname}:${port}` : location.href;

const createURL = (url: string) => new URL(url, base).toString();

export const API = {
  is_ready: createURL("/api/is_ready"),
  search: createURL("/api/search"),
  meta: createURL("/api/meta"),
};

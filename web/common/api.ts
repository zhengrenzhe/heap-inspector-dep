const port = new URL(location.href).searchParams.get("port");

const base = port ? `http://${location.hostname}:${port}` : location.href;

export const API = {
  is_ready: new URL("/api/is_ready", base).toString(),
};

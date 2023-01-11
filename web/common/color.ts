import { theme } from "antd";

export function useColor() {
  return theme.useToken().token;
}

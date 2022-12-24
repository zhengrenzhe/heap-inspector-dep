import React, { ReactNode } from "react";
import { Grid, Input } from "@mantine/core";
import { ColSpan } from "@mantine/core/lib/Grid/Col/Col.styles";

interface IBlockProps {
  label: string;
  description?: string;
  left: ReactNode;
  leftSpan?: ColSpan;
  right: ReactNode;
  rightSpan?: ColSpan;
  cols?: number;
}

export function Block(props: IBlockProps) {
  return (
    <Input.Wrapper label={props.label} description={props.description} mb={18}>
      <Grid mt={0} gutter="md">
        <Grid.Col span={props.leftSpan ?? 2}>{props.left}</Grid.Col>
        <Grid.Col span={props.rightSpan ?? 4}>{props.right}</Grid.Col>
      </Grid>
    </Input.Wrapper>
  );
}

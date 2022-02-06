import React, { Component } from "react";
import { observer } from "mobx-react";
import {
  TextInput,
  Paper,
  MultiSelect,
  SegmentedControl,
  InputWrapper,
  NumberInput,
  Checkbox,
  Button,
  Grid,
} from "@mantine/core";

import { I18n, i18n } from "@/i18n";
import { CompareMode, filter_from, IFilterCondition } from "@/types";
import { ParserService } from "@/service";
import { inject } from "@/util";

interface IFilterPanelProps {
  onSubmit: () => void;
}

const filter_from_data = filter_from.map((f) => ({
  value: f,
  label: i18n(f),
}));

const compare_data = [
  { value: CompareMode.LessThan.toString(), label: i18n("less-than") },
  { value: CompareMode.MoreThan.toString(), label: i18n("more-than") },
];

@observer
export class FilterPanel extends Component<IFilterPanelProps> {
  @inject()
  private parserService!: ParserService;

  private get vm() {
    return this.parserService.viewModel;
  }

  private renderCompare(
    label: keyof I18n,
    description: keyof I18n,
    compare_field: keyof IFilterCondition,
    value_field: keyof IFilterCondition
  ) {
    return (
      <InputWrapper label={i18n(label)} description={i18n(description)}>
        <Grid columns={24}>
          <Grid.Col span={8}>
            <SegmentedControl
              data={compare_data}
              size="xs"
              transitionDuration={300}
              transitionTimingFunction="linear"
              value={this.vm.filter[compare_field] as string}
              onChange={(e) => this.vm.setFilter(compare_field, parseInt(e))}
              style={{ width: "100%" }}
            />
          </Grid.Col>
          <Grid.Col span={16}>
            <NumberInput
              size="xs"
              value={this.vm.filter[value_field] as number}
              min={0}
              onChange={(val) => val && this.vm.setFilter(value_field, val)}
            />
          </Grid.Col>
        </Grid>
      </InputWrapper>
    );
  }

  public render() {
    return (
      <Paper padding="md" shadow="sm" radius="md">
        <InputWrapper
          label={i18n("filter_source")}
          description={i18n("filter_source_desc")}
        >
          <Grid columns={24}>
            <Grid.Col span={12}>
              <MultiSelect
                data={filter_from_data}
                value={this.vm.filter.filter_from}
                size="xs"
                clearable
                onChange={(val) => this.vm.setFilter("filter_from", val)}
                style={{ width: 150 }}
              />
            </Grid.Col>

            <Grid.Col span={12}>
              <TextInput
                size="xs"
                value={this.vm.filter.filter_name}
                onChange={(e) =>
                  this.vm.setFilter("filter_name", e.target.value)
                }
              />
            </Grid.Col>
          </Grid>
        </InputWrapper>

        {this.renderCompare(
          "self_size",
          "self_size_desc",
          "self_size_compare_mode",
          "self_size"
        )}

        {this.renderCompare(
          "retain_size",
          "retain_size_desc",
          "retain_size_compare_mode",
          "retain_size"
        )}

        {this.renderCompare(
          "reference_depth",
          "reference_depth_desc",
          "reference_depth_compare_mode",
          "reference_depth"
        )}

        <InputWrapper
          label={i18n("nodes_limit")}
          description={i18n("nodes_limit_desc")}
        >
          <NumberInput
            size="xs"
            value={this.vm.filter.nodes_limit}
            min={0}
            onChange={(val) => val && this.vm.setFilter("nodes_limit", val)}
            style={{ width: 200 }}
          />
        </InputWrapper>

        <InputWrapper
          label={i18n("ignore_system_node")}
          description={i18n("ignore_system_node_desc")}
        >
          <Checkbox
            checked={this.vm.filter.ignore_system_node}
            size="xs"
            label={i18n(
              this.vm.filter.ignore_system_node ? "hidden" : "display"
            )}
            onChange={(val) =>
              this.vm.setFilter("ignore_system_node", val.target.checked)
            }
          />
        </InputWrapper>

        <Button variant="light" onClick={this.props.onSubmit}>
          {i18n("apply-filter")}
        </Button>
      </Paper>
    );
  }
}

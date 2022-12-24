import React, { Component } from "react";
import {
  Button,
  MultiSelect,
  NumberInput,
  SegmentedControl,
  TextInput,
} from "@mantine/core";
import { observer } from "mobx-react";
import { BsSearch } from "react-icons/bs";

import { __, inject } from "@web/common";
import { OmniService } from "@web/page/canvas/omnibox/omniService";
import { Block } from "@web/page/canvas/omnibox/block";

@observer
export class FilterPanel extends Component {
  @inject()
  private omniService: OmniService;

  public override render() {
    return (
      <div>
        <Block
          label={__("source")}
          description={__("source-desc")}
          leftSpan={6}
          left={
            <MultiSelect
              data={[
                { label: __("constructor_name"), value: "constructor_name" },
                { label: __("closure_name"), value: "closure_name" },
                { label: __("string_value"), value: "string_value" },
              ]}
              value={this.omniService.viewModel.filter.filter_from}
              size="xs"
              clearable
              onChange={(val) =>
                this.omniService.viewModel.setFilter("filter_from", val)
              }
            />
          }
          right={
            <TextInput
              size="xs"
              value={this.omniService.viewModel.filter.filter_name}
              onChange={(e) =>
                this.omniService.viewModel.setFilter(
                  "filter_name",
                  e.target.value
                )
              }
            />
          }
          rightSpan="auto"
        />

        <Block
          label={__("self_size")}
          description={__("self_size_desc")}
          cols={3}
          left={
            <SegmentedControl
              size={"xs"}
              value={this.omniService.viewModel.filter.self_size_mode}
              data={[
                { label: __("less_than"), value: "less_than" },
                { label: __("more_than"), value: "more_than" },
              ]}
              onChange={(val) =>
                this.omniService.viewModel.setFilter("self_size_mode", val)
              }
            />
          }
          leftSpan="content"
          right={
            <NumberInput
              size="xs"
              value={this.omniService.viewModel.filter.self_size}
              onChange={(val) =>
                this.omniService.viewModel.setFilter("self_size", val ?? 0)
              }
            />
          }
          rightSpan="auto"
        />

        <Block
          label={__("retained_size")}
          description={__("retained_size_desc")}
          cols={3}
          left={
            <SegmentedControl
              size={"xs"}
              value={this.omniService.viewModel.filter.retained_size_mode}
              data={[
                { label: __("less_than"), value: "less_than" },
                { label: __("more_than"), value: "more_than" },
              ]}
              onChange={(val) =>
                this.omniService.viewModel.setFilter("retained_size_mode", val)
              }
            />
          }
          leftSpan="content"
          right={
            <NumberInput
              size="xs"
              value={this.omniService.viewModel.filter.retained_size}
              onChange={(val) =>
                this.omniService.viewModel.setFilter("retained_size", val ?? 0)
              }
            />
          }
          rightSpan="auto"
        />

        <Block
          label={__("depth")}
          description={__("depth_desc")}
          cols={3}
          left={
            <NumberInput
              size="xs"
              value={this.omniService.viewModel.filter.depth}
              onChange={(val) =>
                this.omniService.viewModel.setFilter("depth", val ?? 0)
              }
            />
          }
          leftSpan={"auto"}
          right={null}
        />

        <Button
          leftIcon={<BsSearch />}
          variant="light"
          color="teal"
          w={140}
          size="xs"
        >
          {__("apply_filter")}
        </Button>
      </div>
    );
  }
}

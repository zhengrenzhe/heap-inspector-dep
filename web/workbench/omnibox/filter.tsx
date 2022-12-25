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
import { Block } from "@web/workbench/omnibox/block";
import { OmniService } from "@web/workbench/omnibox/omniService";

@observer
export class FilterPanel extends Component {
  @inject()
  private omniService: OmniService;

  private get vm() {
    return this.omniService.viewModel;
  }

  private get meta() {
    return this.vm.meta;
  }

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
              value={this.vm.filter.filter_from}
              size="xs"
              clearable
              onChange={(val) => this.vm.setFilter("filter_from", val)}
            />
          }
          right={
            <TextInput
              size="xs"
              value={this.vm.filter.filter_name}
              onChange={(e) => this.vm.setFilter("filter_name", e.target.value)}
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
              value={this.vm.filter.self_size_mode}
              color="teal"
              data={[
                { label: __("less_than"), value: "less_than" },
                { label: __("more_than"), value: "more_than" },
              ]}
              onChange={(val) => this.vm.setFilter("self_size_mode", val)}
            />
          }
          leftSpan="content"
          right={
            <NumberInput
              size="xs"
              value={this.vm.filter.self_size}
              onChange={(val) => this.vm.setFilter("self_size", val ?? 0)}
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
              value={this.vm.filter.retained_size_mode}
              color="teal"
              data={[
                { label: __("less_than"), value: "less_than" },
                { label: __("more_than"), value: "more_than" },
              ]}
              onChange={(val) => this.vm.setFilter("retained_size_mode", val)}
            />
          }
          leftSpan="content"
          right={
            <NumberInput
              size="xs"
              value={this.vm.filter.retained_size}
              onChange={(val) => this.vm.setFilter("retained_size", val ?? 0)}
            />
          }
          rightSpan="auto"
        />

        <Block
          label={__("depth")}
          description={__("depth_desc")}
          cols={3}
          left={
            <SegmentedControl
              size={"xs"}
              value={this.vm.filter.depth_mode}
              color="teal"
              data={[
                { label: __("less_than"), value: "less_than" },
                { label: __("more_than"), value: "more_than" },
              ]}
              onChange={(val) => this.vm.setFilter("depth_mode", val)}
            />
          }
          leftSpan="content"
          right={
            <NumberInput
              size="xs"
              value={this.vm.filter.depth}
              onChange={(val) => this.vm.setFilter("depth", val ?? 0)}
            />
          }
          rightSpan="auto"
        />

        {this.meta ? (
          <Block
            label={__("node_types")}
            description={__("node_types_desc")}
            cols={3}
            left={
              <MultiSelect
                data={this.meta.node_types.map((t) => ({ label: t, value: t }))}
                value={this.vm.filter.node_types}
                size="xs"
                clearable
                onChange={(val) => this.vm.setFilter("node_types", val)}
              />
            }
            leftSpan={"auto"}
          />
        ) : null}

        <Button
          leftIcon={<BsSearch />}
          variant="light"
          color="teal"
          w={140}
          size="xs"
          loading={this.vm.searching}
          onClick={() => this.omniService.search()}
        >
          {__("apply_filter")}
        </Button>
      </div>
    );
  }
}

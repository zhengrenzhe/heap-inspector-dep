import React, { Component } from "react";
import { Input, MultiSelect, SimpleGrid, TextInput } from "@mantine/core";
import { observer } from "mobx-react";

import { __, inject } from "@web/common";
import { OmniService } from "@web/page/canvas/omnibox/omniService";

@observer
export class FilterPanel extends Component {
  @inject()
  private omniService: OmniService;

  public override render() {
    return (
      <div>
        <Input.Wrapper label={__("source")} description={__("source-desc")}>
          <SimpleGrid cols={2} mt={6}>
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
          </SimpleGrid>
        </Input.Wrapper>
      </div>
    );
  }
}

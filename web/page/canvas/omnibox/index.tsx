import React, { Component } from "react";
import { Paper, Tabs } from "@mantine/core";
import { observer } from "mobx-react";

import { __, inject } from "@web/common";
import { FilterPanel } from "@web/page/canvas/omnibox/filter";
import { RepeatedString } from "@web/page/canvas/omnibox/repeated_string";
import { OmniService } from "@web/page/canvas/omnibox/omniService";

@observer
export class Omnibox extends Component {
  @inject()
  private omniService: OmniService;

  private panels = [
    {
      key: "filter",
      title: __("filter"),
      comp: <FilterPanel />,
    },
    {
      key: "repeated_string",
      title: __("repeated_string"),
      comp: <RepeatedString />,
    },
  ];

  public override render() {
    return (
      <Paper shadow="sm" radius="md" p="md" className="omnibox">
        <Tabs
          radius="md"
          defaultValue={this.panels[0]?.key ?? ""}
          onTabChange={(mode) =>
            this.omniService.viewModel.setMode(mode as string)
          }
        >
          <Tabs.List>
            {this.panels.map((p) => (
              <Tabs.Tab value={p.key} key={p.key} color="teal">
                {p.title}
              </Tabs.Tab>
            ))}
          </Tabs.List>

          {this.panels.map((p) => (
            <Tabs.Panel value={p.key} pt="xs" key={p.key}>
              {p.comp}
            </Tabs.Panel>
          ))}
        </Tabs>
      </Paper>
    );
  }
}

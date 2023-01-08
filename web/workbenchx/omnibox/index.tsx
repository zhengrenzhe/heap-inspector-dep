import React, { Component } from "react";
import { Paper, Tabs } from "@mantine/core";
import { observer } from "mobx-react";

import { __, inject } from "@web/common";
import { Meta } from "@web/workbenchx/omnibox/meta";
import { FilterPanel } from "@web/workbenchx/omnibox/filter";
import { RepeatedString } from "@web/workbenchx/omnibox/repeated_string";
import { OmniService } from "@web/workbenchx/omnibox/omniService";

import "./omnibox.less";

@observer
export class Omnibox extends Component {
  @inject()
  private omniService: OmniService;

  private panels = [
    {
      key: "search",
      title: __("search"),
      comp: <FilterPanel />,
    },
    {
      key: "repeated_string",
      title: __("repeated_string"),
      comp: <RepeatedString />,
    },
    {
      key: "statistics",
      title: __("statistics"),
      comp: <Meta />,
    },
  ];

  public override componentDidMount() {
    this.omniService.init();
  }

  public override render() {
    return (
      <div className="omnibox">
        <Paper shadow="md" radius="md" p="md">
          <Tabs radius="md" defaultValue={this.panels[0]?.key ?? ""}>
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
      </div>
    );
  }
}

import React, { Component } from "react";
import { Paper, Tabs } from "@mantine/core";
import { observer } from "mobx-react";

import { __ } from "@web/common";
import { FilterPanel } from "@web/page/canvas/omnibox/filter";
import { RepeatedString } from "@web/page/canvas/omnibox/repeated_string";
import { Meta } from "@web/page/canvas/omnibox/meta";

@observer
export class Omnibox extends Component {
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
  ];

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

        <Paper shadow="md" radius="md" p="md" mt={20}>
          <Meta />
        </Paper>
      </div>
    );
  }
}

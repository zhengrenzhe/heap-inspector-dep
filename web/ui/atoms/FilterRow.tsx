import React, { ChangeEventHandler, Component } from "react";
import { observer } from "mobx-react";

import { CompareMode, IFilterCondition } from "@/types";
import { inject, toNumber } from "@/util";
import { I18n, i18n } from "@/i18n";
import { ParserService } from "@/service";

interface IRowProps {
  field: keyof IFilterCondition;
  placeholder?: keyof I18n;
  inputIsNumber?: boolean;
  inputType?: "text" | "number" | "checkbox";
  selectField?: keyof IFilterCondition;
  compareMode?: boolean;
  inputIsCheckbox?: boolean;
}

@observer
export class FilterRow extends Component<IRowProps> {
  @inject()
  private parserService!: ParserService;

  private get value() {
    return this.parserService.viewModel.filter[this.props.field];
  }

  private get selectFieldValue() {
    if (this.props.selectField) {
      return this.parserService.viewModel.filter[this.props.selectField];
    }
  }

  private onInputChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    this.parserService.viewModel.setFilter(
      this.props.field,
      this.props.inputIsNumber
        ? toNumber(e.target.value)
        : this.props.inputIsCheckbox
        ? e.target.checked
        : e.target.value
    );
  };

  private onSelectChange: ChangeEventHandler<HTMLSelectElement> = (e) => {
    if (this.props.selectField) {
      const val = toNumber(e.target.value);
      this.parserService.viewModel.setFilter(
        this.props.selectField,
        val === CompareMode.MoreThan
          ? CompareMode.MoreThan
          : CompareMode.LessThan
      );
    }
  };

  public render() {
    return (
      <div className="row">
        <label htmlFor={this.props.field} className="filter-label">
          {i18n(this.props.field as keyof I18n)}
        </label>

        {this.props.compareMode && this.props.selectField ? (
          <select
            className="filter-select"
            onChange={this.onSelectChange}
            value={this.selectFieldValue as string}
          >
            {[
              { value: CompareMode.MoreThan, name: "more-than" as keyof I18n },
              { value: CompareMode.LessThan, name: "less-than" as keyof I18n },
            ].map((s, i) => (
              <option value={s.value} key={i}>
                {i18n(s.name)}
              </option>
            ))}
          </select>
        ) : null}

        <input
          type={this.props.inputType || "text"}
          id={this.props.field}
          min={0}
          className="filter-input"
          placeholder={
            this.props.placeholder ? i18n(this.props.placeholder) : ""
          }
          value={this.value as string}
          onChange={this.onInputChange}
          checked={this.value as boolean}
        />
      </div>
    );
  }
}

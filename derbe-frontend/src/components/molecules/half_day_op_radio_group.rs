use crate::components::atoms::dds_radio_input::DDSRadioInput;
use derbe_shared::enums::HalfDayOptions;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub duration_gt_one: bool,
    pub selected: HalfDayOptions,
    pub on_change: Callback<HalfDayOptions>,
}

#[function_component(HalfDayOpRadioGroup)]
pub fn dds_radio_group(props: &Props) -> Html {
    let props_on_change = props.on_change.clone();

    let half_day_opt_onchange = {
        Callback::from(move |half_day_op: HalfDayOptions| {
            props_on_change.emit(half_day_op);
        })
    };

    html! {
        <fieldset>
            <DDSRadioInput
                id="allDaysFull"
                name="halfDayOps"
                label="All days are full"
                checked={props.selected == HalfDayOptions::FullDays}
                disabled={false}
                on_change={half_day_opt_onchange.clone()}
            />
            <DDSRadioInput
                id="firstDayHalf"
                name="halfDayOps"
                label="First day is a half day"
                checked={props.selected == HalfDayOptions::FirstDayHalf}
                disabled={false}
                on_change={half_day_opt_onchange.clone()}
            />
            <DDSRadioInput
                id="lastDayHalf"
                name="halfDayOps"
                label="Last day is a half day"
                checked={props.selected == HalfDayOptions::LastDayHalf}
                disabled={props.duration_gt_one}
                on_change={half_day_opt_onchange.clone()}
            />
            <DDSRadioInput
                id="firstAndLastHalfs"
                name="halfDayOps"
                label="First and last days are half days"
                checked={props.selected == HalfDayOptions::BothHalfDays}
                disabled={props.duration_gt_one}
                on_change={half_day_opt_onchange.clone()}
            />
        </fieldset>

    }
}

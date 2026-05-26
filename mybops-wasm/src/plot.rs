use arrow::{
    array::{AsArray, RecordBatch},
    compute,
    csv::Writer,
    datatypes::{DataType, Float64Type, UInt32Type},
    util::display,
};
use leptos::{either::Either, prelude::*};
use plotters::prelude::{
    BLACK, ChartBuilder, Circle, Color, Histogram, IntoDrawingArea, IntoSegmentedCoord, LineSeries,
    RED, WHITE,
};
use plotters_canvas::CanvasBackend;
use std::{collections::HashMap, sync::Arc};

use crate::dataframe::DataFrame;

#[derive(Clone, Debug)]
pub enum DataView {
    Table,
    ColumnGraph,
    LineGraph,
    ScatterPlot,
    CumLineGraph,
    Csv,
}

#[component]
pub fn DataViewRender(
    view: ReadSignal<DataView>,
    #[prop(into)] df: Signal<DataFrame>,
    set_error: WriteSignal<Option<String>>,
) -> impl IntoView {
    Effect::new(move || {
        let df = &*df.read();
        if let Err(e) = match view.get() {
            DataView::Table | DataView::Csv => Ok(()),
            DataView::ColumnGraph => draw_column_graph(df),
            DataView::LineGraph => draw_line_graph(df),
            DataView::ScatterPlot => draw_scatter_plot(df),
            DataView::CumLineGraph => draw_cum_line_graph(df),
        } {
            set_error.set(Some(e.to_string()))
        }
    });
    view! {
      <div>
        <canvas
          id="canvas"
          width="640"
          height="426"
          class=(["hidden"], move || matches!(view.get(), DataView::Table | DataView::Csv))
        ></canvas>
        {move || {
          if let DataView::Table = view.get() {
            df_table_view(&df.read(), true).into_any()
          } else if let DataView::Csv = view.get() {
            let csv = write_csv(&df.read());
            view! {
              <p>
                {csv
                  .lines()
                  .map(|items| Either::Left(view! { {items} }))
                  .intersperse(Either::Right(view! { <br /> }))
                  .collect_view()}
              </p>
            }
              .into_any()
          } else {
            ().into_any()
          }
        }}
      </div>
    }
}

pub fn df_table_view(df: &DataFrame, min_width: bool) -> impl IntoView {
    let style = if min_width {
        "min-width: calc(min(568px, 100%))"
    } else {
        "min-width: 100%"
    };
    view! {
      <table style=style>
        <thead>
          <tr>
            <th class="px-3 border-b border-gray-200">"#"</th>
            {df
              .schema
              .fields
              .iter()
              .map(|f| {
                view! {
                  <th class="pr-8 text-left border-b border-gray-200">{f.name().clone()}</th>
                }
              })
              .collect_view()}
          </tr>
        </thead>
        <tbody>{(0..df.arrays[0].len()).map(|i| df_item_view(df, i)).collect_view()}</tbody>
      </table>
    }
}

fn df_item_view(df: &DataFrame, i: usize) -> impl IntoView {
    view! {
      <tr>
        <th class="border-b border-gray-100">{i + 1}</th>
        {df
          .arrays
          .iter()
          .map(|item| {
            view! {
              <td class="pr-8 border-b border-gray-100">
                {display::array_value_to_string(item, i).unwrap()}
              </td>
            }
          })
          .collect_view()}
      </tr>
    }
}

fn draw_column_graph(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::new("canvas").expect("cannot find canvas");
    let root = backend.into_drawing_area();

    root.fill(&WHITE)?;

    let mut builder = ChartBuilder::on(&root);
    builder
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5);
    let range = compute::cast(&df.arrays[1], &DataType::Float64).unwrap();
    let range = range.as_primitive::<Float64Type>();
    match df.arrays[0].data_type() {
        DataType::Int64 | DataType::UInt64 => {
            let domain = compute::cast(&df.arrays[0], &DataType::UInt32).unwrap();
            let domain = domain.as_primitive::<UInt32Type>();
            let mut data = HashMap::new();
            for (i, f) in domain.iter().zip(range) {
                *data.entry(i.unwrap()).or_insert(0f64) += f.unwrap();
            }
            let domain = 0u32..compute::max(domain).unwrap();
            let mut chart = builder.build_cartesian_2d(
                domain.into_segmented(),
                0f64..*data.values().max_by(|a, b| a.total_cmp(b)).unwrap(),
            )?;
            chart
                .configure_mesh()
                .disable_x_mesh()
                .bold_line_style(WHITE.mix(0.3))
                .y_desc(df.schema.fields[1].name())
                .x_desc(df.schema.fields[0].name())
                .axis_desc_style(("sans-serif", 15))
                .draw()?;
            chart.draw_series(
                Histogram::vertical(&chart)
                    .style(RED.mix(0.5).filled())
                    .data(data.into_iter()),
            )?;
        }
        DataType::LargeUtf8 => {
            let domain: Vec<_> = df.arrays[0]
                .as_string::<i64>()
                .into_iter()
                .map(Option::unwrap)
                .collect();
            let range: Vec<_> = range.into_iter().map(Option::unwrap).collect();
            let mut chart = builder.build_cartesian_2d(
                domain.into_segmented(),
                0f64..*range.iter().max_by(|a, b| a.total_cmp(b)).unwrap(),
            )?;
            chart
                .configure_mesh()
                .disable_x_mesh()
                .bold_line_style(WHITE.mix(0.3))
                .y_desc(df.schema.fields[1].name())
                .x_desc(df.schema.fields[0].name())
                .axis_desc_style(("sans-serif", 15))
                .draw()?;
            chart.draw_series(
                Histogram::vertical(&chart)
                    .style(RED.mix(0.5).filled())
                    .data(domain.iter().zip(range.into_iter())),
            )?;
        }
        _ => todo!(),
    }
    Ok(())
}

fn draw_line_graph(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::new("canvas").expect("cannot find canvas");
    let root = backend.into_drawing_area();

    root.fill(&WHITE)?;

    let mut builder = ChartBuilder::on(&root);
    builder
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5);
    let data = df_coords(df)?;
    let mut chart = builder.build_cartesian_2d(
        0f64..compute::max(
            compute::cast(&df.arrays[0], &DataType::Float64)
                .unwrap()
                .as_primitive::<Float64Type>(),
        )
        .unwrap(),
        0f64..compute::max(
            compute::cast(&df.arrays[1], &DataType::Float64)
                .unwrap()
                .as_primitive::<Float64Type>(),
        )
        .unwrap(),
    )?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(data, BLACK))?;
    Ok(())
}

fn draw_scatter_plot(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::new("canvas").expect("cannot find canvas");
    let root = backend.into_drawing_area();

    root.fill(&WHITE)?;

    let mut builder = ChartBuilder::on(&root);
    builder
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5);
    let data = df_coords(df)?;
    let mut chart = builder.build_cartesian_2d(
        0f64..compute::max(
            compute::cast(&df.arrays[0], &DataType::Float64)
                .unwrap()
                .as_primitive::<Float64Type>(),
        )
        .unwrap(),
        0f64..compute::max(
            compute::cast(&df.arrays[1], &DataType::Float64)
                .unwrap()
                .as_primitive::<Float64Type>(),
        )
        .unwrap(),
    )?;
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;
    chart.draw_series(data.into_iter().map(|c| Circle::new(c, 2, BLACK.filled())))?;
    Ok(())
}

fn draw_cum_line_graph(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::new("canvas").expect("cannot find canvas");
    let root = backend.into_drawing_area();

    root.fill(&WHITE)?;

    let mut builder = ChartBuilder::on(&root);
    builder
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5);
    let df = df_coords(df)?;
    let mut cum_sum = 0.0;
    let mut data = Vec::with_capacity(2 * df.len());
    let mut max = 0.0;
    for (d, f) in df {
        data.push((cum_sum, f));
        cum_sum += d;
        data.push((cum_sum, f));
        max = f64::max(max, f);
    }
    let mut chart = builder.build_cartesian_2d(0f64..cum_sum, 0f64..max)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(data.into_iter(), BLACK))?;
    Ok(())
}

fn df_coords(df: &DataFrame) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    let domain = compute::cast(&df.arrays[0], &DataType::Float64).unwrap();
    let domain = domain.as_primitive::<Float64Type>();
    let range = compute::cast(&df.arrays[1], &DataType::Float64).unwrap();
    let range = range.as_primitive::<Float64Type>();
    domain
        .into_iter()
        .zip(range)
        .map(|(o1, o2)| {
            Ok((
                o1.ok_or(format!(
                    "unsupported data type for line graph: {:?}",
                    df.arrays[0].data_type()
                ))?,
                o2.ok_or(format!(
                    "unsupported data type for line graph: {:?}",
                    df.arrays[1].data_type()
                ))?,
            ))
        })
        .collect()
}

fn write_csv(df: &DataFrame) -> String {
    let output = Vec::new();
    let mut writer = Writer::new(output);
    writer
        .write(&RecordBatch::try_new(Arc::clone(&df.schema), df.arrays.clone()).unwrap())
        .unwrap();
    String::from_utf8(writer.into_inner()).unwrap()
}

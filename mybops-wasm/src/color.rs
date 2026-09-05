use leptos::prelude::*;

use crate::base::SelectWithCallback;

#[derive(PartialEq)]
enum Sort {
    Name,
    Hex,
    L,
    C,
    H,
}

#[component]
pub fn Color() -> impl IntoView {
    let css = [
        ("aliceblue", "#f0f8ff"),
        ("antiquewhite", "#faebd7"),
        ("aqua", "#00ffff"),
        ("aquamarine", "#7fffd4"),
        ("azure", "#f0ffff"),
        ("beige", "#f5f5dc"),
        ("bisque", "#ffe4c4"),
        ("black", "#000000"),
        ("blanchedalmond", "#ffebcd"),
        ("blue", "#0000ff"),
        ("blueviolet", "#8a2be2"),
        ("brown", "#a52a2a"),
        ("burlywood", "#deb887"),
        ("cadetblue", "#5f9ea0"),
        ("chartreuse", "#7fff00"),
        ("chocolate", "#d2691e"),
        ("coral", "#ff7f50"),
        ("cornflowerblue", "#6495ed"),
        ("cornsilk", "#fff8dc"),
        ("crimson", "#dc143c"),
        ("cyan", "#00ffff"),
        ("darkblue", "#00008b"),
        ("darkcyan", "#008b8b"),
        ("darkgoldenrod", "#b8860b"),
        ("darkgray", "#a9a9a9"),
        ("darkgreen", "#006400"),
        ("darkgrey", "#a9a9a9"),
        ("darkkhaki", "#bdb76b"),
        ("darkmagenta", "#8b008b"),
        ("darkolivegreen", "#556b2f"),
        ("darkorange", "#ff8c00"),
        ("darkorchid", "#9932cc"),
        ("darkred", "#8b0000"),
        ("darksalmon", "#e9967a"),
        ("darkseagreen", "#8fbc8f"),
        ("darkslateblue", "#483d8b"),
        ("darkslategray", "#2f4f4f"),
        ("darkslategrey", "#2f4f4f"),
        ("darkturquoise", "#00ced1"),
        ("darkviolet", "#9400d3"),
        ("deeppink", "#ff1493"),
        ("deepskyblue", "#00bfff"),
        ("dimgray", "#696969"),
        ("dimgrey", "#696969"),
        ("dodgerblue", "#1e90ff"),
        ("firebrick", "#b22222"),
        ("floralwhite", "#fffaf0"),
        ("forestgreen", "#228b22"),
        ("fuchsia", "#ff00ff"),
        ("gainsboro", "#dcdcdc"),
        ("ghostwhite", "#f8f8ff"),
        ("gold", "#ffd700"),
        ("goldenrod", "#daa520"),
        ("gray", "#808080"),
        ("green", "#008000"),
        ("greenyellow", "#adff2f"),
        ("grey", "#808080"),
        ("honeydew", "#f0fff0"),
        ("hotpink", "#ff69b4"),
        ("indianred", "#cd5c5c"),
        ("indigo", "#4b0082"),
        ("ivory", "#fffff0"),
        ("khaki", "#f0e68c"),
        ("lavender", "#e6e6fa"),
        ("lavenderblush", "#fff0f5"),
        ("lawngreen", "#7cfc00"),
        ("lemonchiffon", "#fffacd"),
        ("lightblue", "#add8e6"),
        ("lightcoral", "#f08080"),
        ("lightcyan", "#e0ffff"),
        ("lightgoldenrodyellow", "#fafad2"),
        ("lightgray", "#d3d3d3"),
        ("lightgreen", "#90ee90"),
        ("lightgrey", "#d3d3d3"),
        ("lightpink", "#ffb6c1"),
        ("lightsalmon", "#ffa07a"),
        ("lightseagreen", "#20b2aa"),
        ("lightskyblue", "#87cefa"),
        ("lightslategray", "#778899"),
        ("lightslategrey", "#778899"),
        ("lightsteelblue", "#b0c4de"),
        ("lightyellow", "#ffffe0"),
        ("lime", "#00ff00"),
        ("limegreen", "#32cd32"),
        ("linen", "#faf0e6"),
        ("magenta", "#ff00ff"),
        ("maroon", "#800000"),
        ("mediumaquamarine", "#66cdaa"),
        ("mediumblue", "#0000cd"),
        ("mediumorchid", "#ba55d3"),
        ("mediumpurple", "#9370db"),
        ("mediumseagreen", "#3cb371"),
        ("mediumslateblue", "#7b68ee"),
        ("mediumspringgreen", "#00fa9a"),
        ("mediumturquoise", "#48d1cc"),
        ("mediumvioletred", "#c71585"),
        ("midnightblue", "#191970"),
        ("mintcream", "#f5fffa"),
        ("mistyrose", "#ffe4e1"),
        ("moccasin", "#ffe4b5"),
        ("navajowhite", "#ffdead"),
        ("navy", "#000080"),
        ("oldlace", "#fdf5e6"),
        ("olive", "#808000"),
        ("olivedrab", "#6b8e23"),
        ("orange", "#ffa500"),
        ("orangered", "#ff4500"),
        ("orchid", "#da70d6"),
        ("palegoldenrod", "#eee8aa"),
        ("palegreen", "#98fb98"),
        ("paleturquoise", "#afeeee"),
        ("palevioletred", "#db7093"),
        ("papayawhip", "#ffefd5"),
        ("peachpuff", "#ffdab9"),
        ("peru", "#cd853f"),
        ("pink", "#ffc0cb"),
        ("plum", "#dda0dd"),
        ("powderblue", "#b0e0e6"),
        ("purple", "#800080"),
        ("rebeccapurple", "#663399"),
        ("red", "#ff0000"),
        ("rosybrown", "#bc8f8f"),
        ("royalblue", "#4169e1"),
        ("saddlebrown", "#8b4513"),
        ("salmon", "#fa8072"),
        ("sandybrown", "#f4a460"),
        ("seagreen", "#2e8b57"),
        ("seashell", "#fff5ee"),
        ("sienna", "#a0522d"),
        ("silver", "#c0c0c0"),
        ("skyblue", "#87ceeb"),
        ("slateblue", "#6a5acd"),
        ("slategray", "#708090"),
        ("slategrey", "#708090"),
        ("snow", "#fffafa"),
        ("springgreen", "#00ff7f"),
        ("steelblue", "#4682b4"),
        ("tan", "#d2b48c"),
        ("teal", "#008080"),
        ("thistle", "#d8bfd8"),
        ("tomato", "#ff6347"),
        ("turquoise", "#40e0d0"),
        ("violet", "#ee82ee"),
        ("wheat", "#f5deb3"),
        ("white", "#ffffff"),
        ("whitesmoke", "#f5f5f5"),
        ("yellow", "#ffff00"),
        ("yellowgreen", "#9acd32"),
    ];
    let tailwind = "Slate
50
#f8fafc
100
#f1f5f9
200
#e2e8f0
300
#cbd5e1
400
#94a3b8
500
#64748b
600
#475569
700
#334155
800
#1e293b
900
#0f172a
950
#020617
Gray
50
#f9fafb
100
#f3f4f6
200
#e5e7eb
300
#d1d5db
400
#9ca3af
500
#6b7280
600
#4b5563
700
#374151
800
#1f2937
900
#111827
950
#030712
Zinc
50
#fafafa
100
#f4f4f5
200
#e4e4e7
300
#d4d4d8
400
#a1a1aa
500
#71717a
600
#52525b
700
#3f3f46
800
#27272a
900
#18181b
950
#09090b
Neutral
50
#fafafa
100
#f5f5f5
200
#e5e5e5
300
#d4d4d4
400
#a3a3a3
500
#737373
600
#525252
700
#404040
800
#262626
900
#171717
950
#0a0a0a
Stone
50
#fafaf9
100
#f5f5f4
200
#e7e5e4
300
#d6d3d1
400
#a8a29e
500
#78716c
600
#57534e
700
#44403c
800
#292524
900
#1c1917
950
#0c0a09
Red
50
#fef2f2
100
#fee2e2
200
#fecaca
300
#fca5a5
400
#f87171
500
#ef4444
600
#dc2626
700
#b91c1c
800
#991b1b
900
#7f1d1d
950
#450a0a
Orange
50
#fff7ed
100
#ffedd5
200
#fed7aa
300
#fdba74
400
#fb923c
500
#f97316
600
#ea580c
700
#c2410c
800
#9a3412
900
#7c2d12
950
#431407
Amber
50
#fffbeb
100
#fef3c7
200
#fde68a
300
#fcd34d
400
#fbbf24
500
#f59e0b
600
#d97706
700
#b45309
800
#92400e
900
#78350f
950
#451a03
Yellow
50
#fefce8
100
#fef9c3
200
#fef08a
300
#fde047
400
#facc15
500
#eab308
600
#ca8a04
700
#a16207
800
#854d0e
900
#713f12
950
#422006
Lime
50
#f7fee7
100
#ecfccb
200
#d9f99d
300
#bef264
400
#a3e635
500
#84cc16
600
#65a30d
700
#4d7c0f
800
#3f6212
900
#365314
950
#1a2e05
Green
50
#f0fdf4
100
#dcfce7
200
#bbf7d0
300
#86efac
400
#4ade80
500
#22c55e
600
#16a34a
700
#15803d
800
#166534
900
#14532d
950
#052e16
Emerald
50
#ecfdf5
100
#d1fae5
200
#a7f3d0
300
#6ee7b7
400
#34d399
500
#10b981
600
#059669
700
#047857
800
#065f46
900
#064e3b
950
#022c22
Teal
50
#f0fdfa
100
#ccfbf1
200
#99f6e4
300
#5eead4
400
#2dd4bf
500
#14b8a6
600
#0d9488
700
#0f766e
800
#115e59
900
#134e4a
950
#042f2e
Cyan
50
#ecfeff
100
#cffafe
200
#a5f3fc
300
#67e8f9
400
#22d3ee
500
#06b6d4
600
#0891b2
700
#0e7490
800
#155e75
900
#164e63
950
#083344
Sky
50
#f0f9ff
100
#e0f2fe
200
#bae6fd
300
#7dd3fc
400
#38bdf8
500
#0ea5e9
600
#0284c7
700
#0369a1
800
#075985
900
#0c4a6e
950
#082f49
Blue
50
#eff6ff
100
#dbeafe
200
#bfdbfe
300
#93c5fd
400
#60a5fa
500
#3b82f6
600
#2563eb
700
#1d4ed8
800
#1e40af
900
#1e3a8a
950
#172554
Indigo
50
#eef2ff
100
#e0e7ff
200
#c7d2fe
300
#a5b4fc
400
#818cf8
500
#6366f1
600
#4f46e5
700
#4338ca
800
#3730a3
900
#312e81
950
#1e1b4b
Violet
50
#f5f3ff
100
#ede9fe
200
#ddd6fe
300
#c4b5fd
400
#a78bfa
500
#8b5cf6
600
#7c3aed
700
#6d28d9
800
#5b21b6
900
#4c1d95
950
#2e1065
Purple
50
#faf5ff
100
#f3e8ff
200
#e9d5ff
300
#d8b4fe
400
#c084fc
500
#a855f7
600
#9333ea
700
#7e22ce
800
#6b21a8
900
#581c87
950
#3b0764
Fuchsia
50
#fdf4ff
100
#fae8ff
200
#f5d0fe
300
#f0abfc
400
#e879f9
500
#d946ef
600
#c026d3
700
#a21caf
800
#86198f
900
#701a75
950
#4a044e
Pink
50
#fdf2f8
100
#fce7f3
200
#fbcfe8
300
#f9a8d4
400
#f472b6
500
#ec4899
600
#db2777
700
#be185d
800
#9d174d
900
#831843
950
#500724
Rose
50
#fff1f2
100
#ffe4e6
200
#fecdd3
300
#fda4af
400
#fb7185
500
#f43f5e
600
#e11d48
700
#be123c
800
#9f1239
900
#881337
950
#4c0519";
    let css: Vec<_> = css
        .into_iter()
        .enumerate()
        .map(|(key, (color, hex))| {
            let rgb = [
                i64::from_str_radix(&hex[1..3], 16).unwrap(),
                i64::from_str_radix(&hex[3..5], 16).unwrap(),
                i64::from_str_radix(&hex[5..7], 16).unwrap(),
            ];
            let lab = to_oklab(to_linear(rgb));
            let mut lch = to_oklch(lab);
            lch[2] = (lch[2] + 2.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
            (key, color.to_owned(), hex, lab, lch)
        })
        .collect();
    let tailwind: Vec<_> = tailwind
        .lines()
        .collect::<Vec<_>>()
        .chunks(23)
        .flat_map(|a| {
            [
                (format!("{}-{}", a[0].to_lowercase(), a[1]), a[2]),
                (format!("{}-{}", a[0].to_lowercase(), a[3]), a[4]),
                (format!("{}-{}", a[0].to_lowercase(), a[5]), a[6]),
                (format!("{}-{}", a[0].to_lowercase(), a[7]), a[8]),
                (format!("{}-{}", a[0].to_lowercase(), a[9]), a[10]),
                (format!("{}-{}", a[0].to_lowercase(), a[11]), a[12]),
                (format!("{}-{}", a[0].to_lowercase(), a[13]), a[14]),
                (format!("{}-{}", a[0].to_lowercase(), a[15]), a[16]),
                (format!("{}-{}", a[0].to_lowercase(), a[17]), a[18]),
                (format!("{}-{}", a[0].to_lowercase(), a[19]), a[20]),
                (format!("{}-{}", a[0].to_lowercase(), a[21]), a[22]),
            ]
        })
        .enumerate()
        .map(|(key, (color, hex))| {
            let rgb = [
                i64::from_str_radix(&hex[1..3], 16).unwrap(),
                i64::from_str_radix(&hex[3..5], 16).unwrap(),
                i64::from_str_radix(&hex[5..7], 16).unwrap(),
            ];
            let lab = to_oklab(to_linear(rgb));
            let mut lch = to_oklch(lab);
            lch[2] = (lch[2] + 2.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
            (key, color, hex, lab, lch)
        })
        .collect();

    let (colors, set_colors) = signal(css.clone());
    let (pin, set_pin) = signal(None::<usize>);

    crate::nav_content(
        view! {
          <a href="#" class="text-gray-100 font-semibold">
            "Color"
          </a>
        },
        view! {
          <div class="w-fit">
            <SelectWithCallback on_change=move |ev| {
              set_colors
                .set(
                  match ev.target().value().as_str() {
                    "CSS" => css.clone(),
                    "Tailwind" => tailwind.clone(),
                    _ => unreachable!(),
                  },
                );
              set_pin.set(None);
            }>
              <option>"CSS"</option>
              <option>"Tailwind"</option>
            </SelectWithCallback>
          </div>
          <ColorTable colors=colors pin=pin set_pin=set_pin/>
        },
    )
}

#[component]
fn ColorTable(
    colors: ReadSignal<Vec<(usize, String, &'static str, [f64; 3], [f64; 3])>>,
    pin: ReadSignal<Option<usize>>,
    set_pin: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let (sort, set_sort) = signal(Sort::Name);
    let (reverse, set_reverse) = signal(false);
    let (select, set_select) = signal(None::<usize>);
    let set_sort = move |s| {
        if *sort.read() == s {
            set_reverse.set(!reverse.get())
        } else {
            set_reverse.set(false);
            set_sort.set(s)
        }
    };

    let rows = move || {
        let colors = colors.read();
        let mut rows: Vec<_> = colors
            .iter()
            .cloned()
            .map(|(key, color, hex, lab, lch)| {
                if let Some(pin) = pin.get() {
                    let pin = &colors[pin];
                    (
                        key,
                        color,
                        hex,
                        lab,
                        lch,
                        ((pin.3[0] - lab[0]).powi(2)
                            + (pin.3[0] - lab[0]).powi(2)
                            + (pin.3[0] - lab[0]).powi(2))
                        .sqrt(),
                    )
                } else {
                    (key, color, hex, lab, lch, 0.0)
                }
            })
            .collect();
        if pin.read().is_some() {
            rows.sort_by(|a, b| a.5.total_cmp(&b.5))
        } else {
            match *sort.read() {
                Sort::Name => rows.sort_by_key(|c| c.1.clone()),
                Sort::Hex => rows.sort_by_key(|c| c.2),
                Sort::L => rows.sort_by(|a, b| a.3[0].total_cmp(&b.3[0])),
                Sort::C => rows.sort_by(|a, b| a.4[1].total_cmp(&b.4[1])),
                Sort::H => rows.sort_by(|a, b| a.4[2].total_cmp(&b.4[2])),
            }
        }
        if reverse.get() {
            rows.reverse();
        }
        rows.iter()
            .enumerate()
            .map(|(i, (key, color, hex, _, [l, c, h], d))| {
                let key = *key;
                view! {
                  <tr
                    class=("hover:bg-gray-100", move || select.read() != Some(i))
                    class=("bg-gray-200", move || select.read() == Some(i))
                    on:click=move |_| {
                      set_select.set(if select.read() == Some(i) { None } else { Some(i) })
                    }
                  >
                    <td>{color.clone()}</td>
                    <td class="px-2">{*hex}</td>
                    <td class="h-full" style=format!("background-color: {}", hex)></td>
                    <td class="px-2 text-right">{format!("{:.1}%", 100.0 * l)}</td>
                    <td class="text-right">{format!("{:.3}", c)}</td>
                    <td class="px-2 text-right">
                      {format!("{:.3}", h / std::f64::consts::PI * 180.0)}
                    </td>
                    {pin
                      .read()
                      .map(|_| {
                        view! { <td>{format!("{:.3}", d)}</td> }
                      })}
                    <td>
                      <button on:click=move |event| {
                        event.stop_propagation();
                        if pin.read() == Some(key) {
                          set_pin.set(None);
                          set_select.set(None);
                        } else {
                          set_pin.set(Some(key));
                          set_select.set(Some(0))
                        }
                      }>
                        <svg
                          xmlns="http://www.w3.org/2000/svg"
                          width="12"
                          height="12"
                          fill="currentColor"
                          class="mt-1"
                          viewBox="0 0 16 16"
                        >
                          <path d="M4.146.146A.5.5 0 0 1 4.5 0h7a.5.5 0 0 1 .5.5c0 .68-.342 1.174-.646 1.479-.126.125-.25.224-.354.298v4.431l.078.048c.203.127.476.314.751.555C12.36 7.775 13 8.527 13 9.5a.5.5 0 0 1-.5.5h-4v4.5c0 .276-.224 1.5-.5 1.5s-.5-1.224-.5-1.5V10h-4a.5.5 0 0 1-.5-.5c0-.973.64-1.725 1.17-2.189A6 6 0 0 1 5 6.708V2.277a3 3 0 0 1-.354-.298C4.342 1.674 4 1.179 4 .5a.5.5 0 0 1 .146-.354" />
                        </svg>
                      </button>
                    </td>
                  </tr>
                }
            })
            .collect_view()
    };
    view! {
          <table>
            <thead>
              <th>
                <button class="w-full text-left" on:click=move |_| set_sort(Sort::Name)>
                  Name
                </button>
              </th>
              <th>
                <button class="px-2 w-full text-left" on:click=move |_| set_sort(Sort::Hex)>
                  Hex
                </button>
              </th>
              <th class="px-1">Color</th>
              <th>
                <button class="w-full" on:click=move |_| set_sort(Sort::L)>
                  L
                </button>
              </th>
              <th>
                <button class="w-full" on:click=move |_| set_sort(Sort::C)>
                  C
                </button>
              </th>
              <th>
                <button class="w-full" on:click=move |_| set_sort(Sort::H)>
                  h
                </button>
              </th>
              {pin.read().map(|_| view! { <th>Delta</th> })}
            </thead>
            {rows}
          </table>
    }
}

fn to_linear(rgb: [i64; 3]) -> [f64; 3] {
    rgb.map(|c| {
        let c = c as f64 / 255.0;
        if c.abs() <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    })
}

fn to_oklab(rgb: [f64; 3]) -> [f64; 3] {
    let [l, m, s] = multiply(
        [
            0.4122214708,
            0.5363325363,
            0.0514459929,
            0.2119034982,
            0.6806995451,
            0.1073969566,
            0.0883024619,
            0.2817188376,
            0.6299787005,
        ],
        rgb,
    );
    let _lms = [l.cbrt(), m.cbrt(), s.cbrt()];
    multiply(
        [
            0.2104542553,
            0.7936177850,
            -0.0040720468,
            1.9779984951,
            -2.4285922050,
            0.4505937099,
            0.0259040371,
            0.7827717662,
            -0.8086757660,
        ],
        _lms,
    )
}

fn to_oklch([l, a, b]: [f64; 3]) -> [f64; 3] {
    [l, (a.powi(2) + b.powi(2)).sqrt(), b.atan2(a)]
}

fn multiply(a: [f64; 9], x: [f64; 3]) -> [f64; 3] {
    [
        a[0] * x[0] + a[1] * x[1] + a[2] * x[2],
        a[3] * x[0] + a[4] * x[1] + a[5] * x[2],
        a[6] * x[0] + a[7] * x[1] + a[8] * x[2],
    ]
}

<p></p><p></p>
<img src="https://raw.githubusercontent.com/griffi-gh/hui/master/.assets/hui.svg" width="120" align="left" alt="logo">
<h1>hUI</h1>
<div>
  <span>
    Simple UI library for games and other interactive applications
  </span><a href="https://crates.io/crates/hui" float="right">
    <img alt="crates.io" src="https://img.shields.io/crates/v/hui.svg?style=flat-square" align="right" height="20">
  </a><br><a href="./LICENSE.txt" align="right" float="right">
    <img alt="license" src="https://img.shields.io/github/license/griffi-gh/hui?style=flat-square" align="right" width="102" height="20">
  </a><span>
    (Formerly <code>kubi-ui</code>)
  </span>
</div>
<p></p>
<br clear="all">

<table align="center">
  <td>
    <img src="https://raw.githubusercontent.com/griffi-gh/hui/master/.assets/demo0.gif" width="300" alt="example: mom_downloader">
  </td>
  <td>
    <img src="https://raw.githubusercontent.com/griffi-gh/hui/master/.assets/demo1.gif" width="300" alt="example: align_test">
  </td>
</table>

<h2>Example</h2>
<img src="https://raw.githubusercontent.com/griffi-gh/hui/master/.assets/exemplaris.png"
  height="175" align="right" float="right" alt="code result">
<pre lang="rust">Container::default()
  .with_size(size!(100%, 50%))
  .with_align(Alignment::Center)
  .with_padding(5.)
  .with_gap(10.)
  .with_background(rect_frame! {
    color: (0.5, 0.5, 0.5, 1.),
    corner_radius: 10.,
  })
  .with_children(|ui| {
    Text::default()
      .with_text("Hello, world")
      .with_text_size(100)
      .with_color(color::BLACK)
      .add_child(ui);
    Container::default()
      .with_padding((10., 20.))
      .with_background(rect_frame! {
        color: color::DARK_RED,
        corner_radius: (2.5, 30., 2.5, 2.5),
      })
      .with_children(|ui| {
        Text::default()
          .with_text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.")
          .with_text_size(24)
          .add_child(ui);
      })
      .add_child(ui);
  })
  .add_root(ui, size);</pre>

<h2>Backends</h2>
<p>
  Latest stable release:&nbsp;
  <a href="https://crates.io/crates/hui" float="right">
    <img alt="crates.io" src="https://img.shields.io/crates/v/hui.svg?style=flat-square&label=&color=0d1117" height="20">
  </a>
</p>
<table>
  <tr>
    <th align="center">
      <code>hui</code>
    </th>
    <th align="center">
      <code>glium</code> (render)
    </th>
    <th align="center">
      <code>winit</code> (platform)
    </th>
    <th align="center">
      <code>wgpu</code> (render)
    </th>
  </tr>
  <tr>
    <td align="center">
      <code>master</code>
    </th>
    <td>
      <code>hui-glium = &lt;master&gt;</code><br>
      <code>glium = "0.34"</code>
    </td>
    <td>
      <code>hui-winit = &lt;master&gt;</code><br>
      <code>winit = "0.30"</code> or <code>winit = "0.29"</code>
    </td>
    <td align="center">(support planned)</td>
  </tr>
  <tr>
    <td align="center">
      <code>0.1.0-alpha.4</code>
    </th>
    <td>
      <code>hui-glium = "0.1.0-alpha.4"</code><br>
      <code>glium = "0.34"</code>
    </td>
    <td>
      <code>hui-winit = "0.1.0-alpha.4"</code><br>
      <code>winit = "0.29"</code>
    </td>
    <td align="center">N/A</td>
  </tr>
  <tr>
    <td align="center">
      <code>0.1.0-alpha.3</code>
    </th>
    <td>
      <code>hui-glium = "0.1.0-alpha.3"</code><br>
      <code>glium = "0.34"</code>
    </td>
    <td align="center" colspan="2">N/A</td>
  </tr>
  <tr>
    <td align="center">
      <code>0.1.0-alpha.2</code>
    </th>
    <td>
      <code>hui-glium = "0.1.0-alpha.2"</code><br>
      <code>glium = "0.34"</code>
    </td>
    <td align="center" colspan="2">N/A</td>
  </tr>
  <tr>
    <td align="center">
      <code>0.1.0-alpha.1</code>
    </th>
    <td>
      <code>hui-glium = "0.1.0-alpha.1"</code><br>
      <code>glium = "0.34"</code>
    </td>
    <td align="center" colspan="2">N/A</td>
  </tr>
  <!-- <tr>
    <td align="center">
      <code>0.0.2</code>
    </th>
    <td>
      <code>hui-glium = "0.0.2"</code><br>
      <code>glium = "0.34"</code>
    </td>
    <td align="center">-</td>
  </tr>
  <tr>
    <td align="center">
      <code>0.0.1</code>
    </th>
    <td>
      <code>hui-glium = "0.0.1"</code><br>
      <code>glium = "0.34"</code>
    </td>
    <td align="center">-</td>
  </tr> -->
</table>

<h2>MSRV</h2>
1.75

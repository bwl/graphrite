use clap::{Parser, Subcommand, Args, ValueEnum};
use graphrite_core::parser::Parser as CoreParser;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Parser)]
#[command(name = "graphrite", version, about = "Graphrite CLI")]
struct Cli { #[command(subcommand)] command: Commands }

#[derive(Subcommand)]
enum Commands { Parse(ParseArgs), Ast(ParseArgs), Check(CheckArgs), Lint(LintArgs), Fmt(FmtArgs), Diag(DiagArgs), Render(RenderArgs) }

#[derive(Args)] struct ParseArgs { input: Option<String>, #[arg(long)] diag_json: bool }
#[derive(Args)] struct CheckArgs { input: Option<String>, #[arg(long)] diag_json: bool }
#[derive(Args)] struct LintArgs { input: Option<String>, #[arg(long)] diag_json: bool, #[arg(long)] pretty: bool }
#[derive(Args)] struct FmtArgs { input: Option<String>, #[arg(long)] write: bool, #[arg(long)] check: bool }
#[derive(Args)] struct DiagArgs { input: Option<String>, #[arg(long)] json: bool }

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)] enum RenderFormat { Dot, Svg }
#[derive(Args)] struct RenderArgs { input: Option<String>, #[arg(long, value_enum, default_value_t = RenderFormat::Dot)] format: RenderFormat }

fn read_input(path: &Option<String>) -> anyhow::Result<String> { if let Some(p)=path{Ok(std::fs::read_to_string(p)?)} else { use std::io::Read; let mut b=String::new(); std::io::stdin().read_to_string(&mut b)?; Ok(b) } }
fn print_pretty(diags:&[graphrite_core::error::Diagnostic]){ for d in diags { if let Some(s)=&d.span { eprintln!("{}:{}:{} {} {}", s.start.line, s.start.col, d.code, d.message, "^"); } else { eprintln!("{} {}", d.code, d.message); } } }

fn render_dot(doc:&graphrite_core::ast::Document)->String{ let mut s=String::new(); s.push_str("digraph G{\n"); for n in &doc.nodes { let l=n.label.replace('"',"\\\""); s.push_str(&format!("  {} [label=\"{}\"];\n",n.id,l)); } for e in &doc.edges { let arrow=match e.kind{graphrite_core::ast::EdgeKind::Flow=>"->",graphrite_core::ast::EdgeKind::Conditional=>"->"}; s.push_str(&format!("  {} {} {};\n",e.from,arrow,e.to)); } s.push_str("}\n"); s }

fn kind_of(label:&str,id:&str)->(&'static str,&'static str,&'static str){ let l=label.to_ascii_lowercase(); if id=="start" { ("stadium","#e6ffe6","#2e7d32") } else if id.starts_with("end")|| l.contains("done")||l.contains("success")||l.contains("fail"){ ("stadium","#eeeeee","#424242") } else if l.contains("? ")||l.ends_with('?')|| l.contains("?\"") { ("diamond","#fff9c4","#f9a825") } else if l.contains("data")||l.contains("store")||l.contains("queue")||l.contains("persist"){ ("cylinder","#e3f2fd","#1565c0") } else if l.contains("error")||l.contains("dead letter"){ ("rect","#ffebee","#c62828") } else { ("rect","#ffffff","#333333") } }

fn render_shape(s:&mut String,shape:&str,x:f32,y:f32,w:f32,h:f32,fill:&str,stroke:&str){ match shape { "rect"=>s.push_str(&format!("  <rect x=\"{:.1}\" y=\"{:.1}\" rx=\"6\" ry=\"6\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" stroke=\"{}\"/>\n",x,y,w,h,fill,stroke)), "stadium"=>{ let r=h/2.0; s.push_str(&format!("  <rect x=\"{:.1}\" y=\"{:.1}\" rx=\"{:.1}\" ry=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" stroke=\"{}\"/>\n",x,y,r,r,w,h,fill,stroke)); }, "diamond"=>{ let cx=x+w/2.0; let cy=y+h/2.0; s.push_str(&format!("  <polygon points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"{}\" stroke=\"{}\"/>\n", cx, y, x+w, cy, cx, y+h, x, cy, fill, stroke)); }, "cylinder"=>{ s.push_str(&format!("  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" stroke=\"{}\"/>\n",x,y,w,h,fill,stroke)); s.push_str(&format!("  <ellipse cx=\"{:.1}\" cy=\"{:.1}\" rx=\"{:.1}\" ry=\"6\" fill=\"{}\" stroke=\"{}\"/>\n", x+w/2.0, y, w/2.0, fill, stroke)); s.push_str(&format!("  <ellipse cx=\"{:.1}\" cy=\"{:.1}\" rx=\"{:.1}\" ry=\"6\" fill=\"{}\" stroke=\"{}\"/>\n", x+w/2.0, y+h, w/2.0, fill, stroke)); }, _=>{} } }

fn layered_positions(doc:&graphrite_core::ast::Document,node_w:f32,node_h:f32,pad_x:f32,pad_y:f32)->BTreeMap<&str,(f32,f32)>{
    let lr = matches!(doc.directives.direction, graphrite_core::ast::Direction::LR);
    let mut adj: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut indeg: BTreeMap<&str, usize> = BTreeMap::new();
    for n in &doc.nodes { adj.insert(&n.id, Vec::new()); indeg.insert(&n.id, 0); }
    for e in &doc.edges { if let (Some(_), Some(_))=(adj.get(e.from.as_str()), adj.get(e.to.as_str())) { adj.get_mut(e.from.as_str()).unwrap().push(&e.to); *indeg.get_mut(e.to.as_str()).unwrap() += 1; } }
    let mut q: VecDeque<&str> = VecDeque::new();
    for (id,d) in &indeg { if *d==0 { q.push_back(id); } }
    if q.is_empty() { if let Some(first)=doc.nodes.first() { q.push_back(&first.id); } }
    let mut layer: BTreeMap<&str, usize> = BTreeMap::new();
    while let Some(u)=q.pop_front(){ let lu=*layer.get(u).unwrap_or(&0); for v in adj.get(u).cloned().unwrap_or_default(){ if layer.get(v).map(|x|*x<lu+1).unwrap_or(true){ layer.insert(v, lu+1); } if let Some(d)=indeg.get_mut(v){ if *d>0 { *d-=1; if *d==0 { q.push_back(v); } } } }
        if !layer.contains_key(u){ layer.insert(u, lu); }
    }
    let mut buckets: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
    for n in &doc.nodes { let l=*layer.get(n.id.as_str()).unwrap_or(&0); buckets.entry(l).or_default().push(&n.id); }
    let mut pos: BTreeMap<&str,(f32,f32)> = BTreeMap::new();
    let mut lidx=0usize;
    for (_l, ids) in buckets.iter() { let count=ids.len() as f32; for (i,id) in ids.iter().enumerate(){ let i=i as f32; let (x,y)= if lr { (pad_x + (lidx as f32)*(node_w+pad_x), pad_y + i*(node_h+pad_y)) } else { (pad_x + i*(node_w+pad_x), pad_y + (lidx as f32)*(node_h+pad_y)) }; pos.insert(id, (x,y)); }
        lidx+=1;
    }
    pos
}

fn render_svg(doc:&graphrite_core::ast::Document)->String{
    let node_w=160f32; let node_h=48f32; let pad_x=90f32; let pad_y=70f32;
    let pos = layered_positions(doc,node_w,node_h,pad_x,pad_y);
    let lr = matches!(doc.directives.direction, graphrite_core::ast::Direction::LR);
    let max_x = pos.values().map(|(x,_)| *x).fold(0.0, f32::max);
    let max_y = pos.values().map(|(_,y)| *y).fold(0.0, f32::max);
    let cols = if lr { ((max_x - pad_x)/(node_w+pad_x)).round()+1.0 } else { ((max_y - pad_y)/(node_h+pad_y)).round()+1.0 };
    let rows = if lr { ((max_y - pad_y)/(node_h+pad_y)).round()+1.0 } else { ((max_x - pad_x)/(node_w+pad_x)).round()+1.0 };
    let width=(cols*(node_w+pad_x)+pad_x) as i32; let height=(rows*(node_h+pad_y)+pad_y+120.0) as i32;
    let mut s=String::new();
    s.push_str(&format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",width,height,width,height));
    s.push_str("  <defs><marker id=\"arrow\" viewBox=\"0 0 10 10\" refX=\"10\" refY=\"5\" markerWidth=\"6\" markerHeight=\"6\" orient=\"auto-start-reverse\"><path d=\"M0 0 L10 5 L0 10 z\" fill=\"#333\"/></marker></defs>\n");
    for e in &doc.edges { if let (Some(&(x1,y1)),Some(&(x2,y2)))=(pos.get(e.from.as_str()),pos.get(e.to.as_str())){ let (sx,sy,tx,ty)= (x1+node_w, y1+node_h/2.0, x2, y2+node_h/2.0); let mid = if lr { (sx+tx)/2.0 } else { (sy+ty)/2.0 }; if lr { s.push_str(&format!("  <polyline fill=\"none\" stroke=\"#333\" stroke-width=\"2\" marker-end=\"url(#arrow)\" points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\"/>\n", sx,sy, mid,sy, tx,ty)); } else { s.push_str(&format!("  <polyline fill=\"none\" stroke=\"#333\" stroke-width=\"2\" marker-end=\"url(#arrow)\" points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\"/>\n", sx,sy, sx,mid, tx,ty)); } } }
    for n in &doc.nodes { if let Some(&(x,y))=pos.get(n.id.as_str()){ let (shape,fill,stroke)=kind_of(&n.label,&n.id); render_shape(&mut s,shape,x,y,node_w,node_h,fill,stroke); s.push_str(&format!("  <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"12\" text-anchor=\"middle\" dominant-baseline=\"middle\">{}</text>\n", x+node_w/2.0, y+node_h/2.0, htmlesc(&n.label))); } }
    let keyx=20.0; let keyy=(height as f32)-110.0; s.push_str(&format!("  <g id=\"legend\">\n    <rect x=\"{:.1}\" y=\"{:.1}\" width=\"520\" height=\"100\" fill=\"#fafafa\" stroke=\"#bbb\"/>\n", keyx,keyy));
    let mut kx=keyx+10.0; let mut ky=keyy+20.0; let items=[ ("Start/End","stadium","#e6ffe6","#2e7d32"),("Decision","diamond","#fff9c4","#f9a825"),("Process","rect","#ffffff","#333333"),("Data/Queue","cylinder","#e3f2fd","#1565c0"),("Error","rect","#ffebee","#c62828") ];
    for (label,shape,fill,stroke) in items { render_shape(&mut s,shape,kx,ky,80.0,30.0,fill,stroke); s.push_str(&format!("    <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"12\">{}</text>\n", kx+90.0, ky+15.0, label)); kx+=200.0; if kx>480.0{ kx=keyx+10.0; ky+=40.0; } }
    s.push_str("  </g>\n</svg>\n"); s }

fn htmlesc(s:&str)->String{ s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;") }

fn main()->anyhow::Result<()> { let cli=Cli::parse(); match cli.command { Commands::Parse(a)|Commands::Ast(a)=>{ let src=read_input(&a.input)?; match CoreParser::parse(&src){ Ok(d)=>{ println!("{}",serde_json::to_string_pretty(&d)?); Ok(()) }, Err(di)=>{ if a.diag_json{ println!("{}",serde_json::to_string_pretty(&di)?);} else { print_pretty(&di);} std::process::exit(1)} } }, Commands::Check(a)=>{ let src=read_input(&a.input)?; match CoreParser::parse(&src){ Ok(_)=>{ println!("ok"); Ok(()) }, Err(di)=>{ if a.diag_json{ println!("{}",serde_json::to_string_pretty(&di)?);} else { print_pretty(&di);} std::process::exit(1)} } }, Commands::Lint(a)=>{ let src=read_input(&a.input)?; match CoreParser::parse(&src){ Ok(_)=>{ println!("clean"); Ok(()) }, Err(di)=>{ if a.diag_json{ println!("{}",serde_json::to_string_pretty(&di)?);} else if a.pretty { print_pretty(&di);} else { for d in di.iter(){ println!("{}\t{}",d.code,d.message);} } std::process::exit(1)} } }, Commands::Fmt(a)=>{ let src=read_input(&a.input)?; match CoreParser::parse(&src){ Ok(_)=>{ let formatted=src.clone(); if a.check{ if formatted!=src { std::process::exit(1);} else { println!("ok"); return Ok(()); } } if a.write{ if let Some(p)=a.input{ std::fs::write(p,formatted)?; } else { print!("{}",formatted);} } else { print!("{}",formatted);} Ok(()) }, Err(di)=>{ print_pretty(&di); std::process::exit(1)} } }, Commands::Diag(a)=>{ let src=read_input(&a.input)?; match CoreParser::parse(&src){ Ok(_)=>{ println!("no issues"); Ok(()) }, Err(di)=>{ if a.json{ println!("{}",serde_json::to_string_pretty(&di)?);} else { print_pretty(&di);} std::process::exit(1)} } }, Commands::Render(a)=>{ let src=read_input(&a.input)?; let doc=match CoreParser::parse(&src){ Ok(d)=>d, Err(di)=>{ print_pretty(&di); std::process::exit(1)} }; match a.format { RenderFormat::Dot=>{ print!("{}",render_dot(&doc)); Ok(()) }, RenderFormat::Svg=>{ print!("{}",render_svg(&doc)); Ok(()) } } } } }

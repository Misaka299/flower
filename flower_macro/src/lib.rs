use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Field, Fields, parse_macro_input};
use syn::parse::Parser;

#[proc_macro_attribute]
pub fn control(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);

    let control_struct;
    let control_base;
    let control_box;

    if let Data::Struct(ref mut data_struct) = input.data {
        if let Fields::Named(ref mut fields) = data_struct.fields {
            let field_initializers = fields.named.iter().map(|field| {
                let field_ident = field.ident.as_ref().unwrap();
                quote! {
                    #field_ident: #field_ident,
                }
            });

            let field_names = fields.named.iter().map(|field| {
                let field_ident = field.ident.as_ref().unwrap();
                quote! {
                    #field_ident
                }
            });

            // 构建字段类型列表
            let field_types = fields.named.iter().map(|field| {
                let field_type = &field.ty;
                quote! {
                    #field_type
                }
            });

            let struct_name = &input.ident;
            let generics = &input.generics;

            control_struct = quote! {
                use flower_base::graphics::rect::Rect;
                impl #generics #struct_name #generics {
                     pub fn create_control(name: impl AsRef<str>, rect: Rect, #(#field_names: #field_types,)*) -> Self {
                        Self {
                            id: 0,
                            name: name.as_ref().to_string(),
                            parent_id: 0,
                            control_type: flower_base::ControlType::Control,
                            base_left: 0.0,
                            base_top: 0.0,
                            rect,
                            child: vec![],
                            visual: false,
                            interactive_state: flower_base::InteractiveState::Ordinary,
                            focus_order: 0,
                            focus: false,
                            non_focus: false,
                            events: Default::default(),
                            #(#field_initializers)*
                        }
                    }
                }
            };

            control_base = quote! {
                use flower_base::event::EventFn;
                use flower_base::InteractiveState;
                use flower_base::control::ControlBase;
                impl ControlBase for #generics #struct_name #generics {
                    fn id(&self) -> i32 { self.id }
                    fn base_left(&self) -> f32 { self.base_left }
                    fn base_top(&self) -> f32 { self.base_top }
                    fn left(&self) -> f32 { self.rect.left }
                    fn top(&self) -> f32 { self.rect.top }
                    fn width(&self) -> f32 { self.rect.width }
                    fn height(&self) -> f32 { self.rect.height }
                    fn child(&mut self) -> &mut Vec<Box<dyn Control>> { &mut self.child }
                    fn visual(&self) -> bool { self.visual }
                    fn interactive_state(&self) -> InteractiveState { self.interactive_state }
                    fn set_interactive_state(&mut self, new_interactive_state: InteractiveState) { self.interactive_state = new_interactive_state }

                    fn set_focus(&mut self) {
                        self.non_focus = true;
                        self.fire_event(EventMessage::FocusGet);
                    }
                    fn cancel_focus(&mut self) {
                        self.non_focus = false;
                        self.fire_event(EventMessage::FocusLost);
                    }

                    fn add_event(&mut self, efn: EventFn) {
                        self.events.entry(efn.into()).or_insert(vec![]).push(efn);
                    }

                    fn add_child(&mut self, child: Box<dyn Control>) {
                        self.child.push(child);
                    }

                    fn focus_order(&self) -> i32 { self.focus_order }

                    /// Find controls in this control and controls under this control whose focus is true
                    fn find_focus_control(&self) -> Option<i32> {
                        if self.focus { return Some(self.id); }
                        for x in &self.child {
                            return x.find_focus_control();
                        }
                        None
                    }

                    fn find_max_order_focus(&self) -> i32 {
                        if self.non_focus == true { return 0; }
                        let mut max = self.focus_order;
                        for ref mut x in &self.child {
                            let i = x.find_max_order_focus();
                            if i > max {
                                max = i;
                            }
                        }
                        max
                    }

                    fn find_min_order_focus(&self) -> i32 {
                        if self.non_focus == true { return 0; }
                        let mut min = self.focus_order;
                        for x in &self.child {
                            let i = x.find_max_order_focus();
                            if i < min {
                                min = i;
                            }
                        }
                        min
                    }

                    fn find_previous_focus_control(&mut self, current_focus_order: i32) -> Option<i32> {
                        let mut loop_index = current_focus_order;
                        let min = self.find_min_order_focus();

                        loop {
                            if loop_index <= min {
                                // If it's the last one, search for the first one
                                loop_index = self.find_max_order_focus();
                            } else {
                                // find previous one
                                loop_index = loop_index - 1;
                            }

                            // check back to the origin
                            if loop_index == current_focus_order {
                                return None;
                            }
                            // check self
                            if loop_index == self.focus_order {
                                return Some(self.id);
                            }
                            // check child
                            match self.search_control_by_focus_order(loop_index) {
                                // The previous one may prohibit focus, so skip it
                                None => { continue; }
                                Some(control) => {
                                    return Some(control.id());
                                }
                            }
                        }
                    }

                    fn find_next_focus_control(&mut self, current_focus_order: i32) -> Option<i32> {
                        let mut loop_index = current_focus_order;
                        let max = self.find_max_order_focus();

                        loop {
                            if loop_index >= max {
                                // if it's the last one, search for the first one
                                loop_index = self.find_min_order_focus();
                            } else {
                                // find previous one
                                loop_index = loop_index + 1;
                            }

                            // check back to the origin
                            if loop_index == current_focus_order {
                                return None;
                            }
                            // check self
                            if loop_index == self.focus_order {
                                return Some(self.id);
                            }
                            // check child
                            match self.search_control_by_focus_order(loop_index) {
                                // The next one may prohibit focus, so skip it
                                None => { continue; }
                                Some(control) => {
                                    return Some(control.id());
                                }
                            }
                        }
                    }

                    fn search_control_by_focus_order(&mut self, order: i32) -> Option<&mut Box<dyn Control>> {
                        match self.child.binary_search_by(|c| c.focus_order().cmp(&order)) {
                            Ok(this_index) => {
                                if self.child.len() - 1 < this_index {
                                    return None;
                                }
                                return Some(&mut self.child[this_index]);
                            }
                            _ => { None }
                        }
                    }

                    /// 获取组件的类型
                    fn control_type(&self) -> flower_base::ControlType {
                        self.control_type.clone()
                    }

                    fn fire_event(&mut self, em: EventMessage) {
                        if let Some(vec) = self.events.get(&em.into()) {
                            for f in vec {
                                match f {
                                    EventFn::LButtonDown(f) => if let EventMessage::LButtonDown(x, y, state) = em { f(x, y, state) },
                                    EventFn::LButtonClick(f) => if let EventMessage::LButtonClick(x, y, state) = em { f(x, y, state) },
                                    EventFn::LButtonUp(f) => if let EventMessage::LButtonUp(x, y, state) = em { f(x, y, state) },
                                    EventFn::RButtonDown(f) => if let EventMessage::RButtonDown(x, y, state) = em { f(x, y, state) },
                                    EventFn::RButtonClick(f) => if let EventMessage::RButtonClick(x, y, state) = em { f(x, y, state) },
                                    EventFn::RButtonUp(f) => if let EventMessage::RButtonUp(x, y, state) = em { f(x, y, state) },
                                    EventFn::MButtonDown(f) => if let EventMessage::MButtonDown(x, y, state) = em { f(x, y, state) },
                                    EventFn::MButtonClick(f) => if let EventMessage::MButtonClick(x, y, state) = em { f(x, y, state) },
                                    EventFn::MButtonUp(f) => if let EventMessage::MButtonUp(x, y, state) = em { f(x, y, state) },
                                    EventFn::OtherButtonDown(f) => if let EventMessage::OtherButtonDown(x, y, state) = em { f(x, y, state) },
                                    EventFn::OtherButtonClick(f) => if let EventMessage::OtherButtonClick(x, y, state) = em { f(x, y, state) },
                                    EventFn::OtherButtonUp(f) => if let EventMessage::OtherButtonUp(x, y, state) = em { f(x, y, state) },
                                    EventFn::MouseEnter(f) => f(),
                                    EventFn::MouseLeave(f) => f(),
                                    EventFn::MouseMove(f) => if let EventMessage::MouseMove(x, y, state) = em { f(x, y, state) },
                                    EventFn::FocusGet(f) => f(),
                                    EventFn::FocusLost(f) => f(),
                                    EventFn::ReSize(_) => {}
                                }
                            }
                        }
                    }
                }
            };

            control_box = quote!{
                impl Into<Box<dyn Control>> for #generics #struct_name #generics {
                    fn into(self) -> Box<dyn Control> {
                        Box::new(self)
                    }
                }
            };

            fields.named.insert(0, Field::parse_named.parse2(quote! {
                /// 控件id
                pub(crate) id: i32
            }).unwrap());
            fields.named.insert(1, Field::parse_named.parse2(quote! {
                /// 控件名称
                pub(crate) name: String
            }).unwrap());
            fields.named.insert(2, Field::parse_named.parse2(quote! {
                /// 父级控件id
                pub(crate) parent_id: i32
            }).unwrap());
            // todo 组件类名
            // fields.named.insert(0, Field::parse_named.parse2(quote! { pub(crate) class: Vec<String>}).unwrap());
            fields.named.insert(3, Field::parse_named.parse2(quote! {
                pub(crate) control_type: flower_base::ControlType
            }).unwrap());
            fields.named.insert(4, Field::parse_named.parse2(quote! {
                pub(crate) base_left: f32
            }).unwrap());
            fields.named.insert(5, Field::parse_named.parse2(quote! {
                pub(crate) base_top: f32
            }).unwrap());
            fields.named.insert(6, Field::parse_named.parse2(quote! {
                pub(crate) rect: Rect
            }).unwrap());
            fields.named.insert(7, Field::parse_named.parse2(quote! {
                /// 子级控件
                pub(crate) child: Vec<Box<dyn Control>>
            }).unwrap());
            fields.named.insert(8, Field::parse_named.parse2(quote! {
                /// 控件可视
                pub(crate) visual: bool
            }).unwrap());
            fields.named.insert(9, Field::parse_named.parse2(quote! {
                /// 交互状态
                pub(crate) interactive_state: InteractiveState
            }).unwrap());
            fields.named.insert(10, Field::parse_named.parse2(quote! {
                /// 焦点顺序，默认使用控件id
                pub(crate) focus_order: i32
            }).unwrap());
            fields.named.insert(11, Field::parse_named.parse2(quote! {
                /// 是否焦点
                pub(crate) focus: bool
            }).unwrap());
            fields.named.insert(12, Field::parse_named.parse2(quote! {
                /// 是否禁止捕获焦点
                pub(crate) non_focus: bool
            }).unwrap());
            fields.named.insert(13, Field::parse_named.parse2(quote! {
                /// 挂载的事件
                pub(crate) events: crate::FxHashMap<flower_base::event::EventType, Vec<flower_base::event::EventFn>>
            }).unwrap());
        } else {
            panic!("control macro does not support structures of this field type");
        }
    } else {
        panic!("control macro can only be used on a struct with named fields");
    }

    let item_tokens = input.into_token_stream();
    return quote! {#item_tokens #control_struct #control_base #control_box}.into();
}
use imgui::*;
use imgui_wgpu::Renderer;
use imgui_winit_support;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod node_editor;

fn main() {
    env_logger::init();

    // Set up window and GPU
    let event_loop = EventLoop::new();
    let (window, instance, mut size, surface, hidpi_factor) = {
        use raw_window_handle::HasRawWindowHandle as _;

        let instance = wgpu::Instance::new();

        let version = env!("CARGO_PKG_VERSION");

        let window = Window::new(&event_loop).unwrap();
        window.set_inner_size(LogicalSize {
            width: 1280.0,
            height: 720.0,
        });
        window.set_title(&format!("imgui-wgpu {}", version));
        let hidpi_factor = window.hidpi_factor();
        let size = window.inner_size().to_physical(hidpi_factor);

        let surface = instance.create_surface(window.raw_window_handle());

        (window, instance, size, surface, hidpi_factor)
    };

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
    });

    let mut device = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    // Set up swap chain
    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::NoVsync,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    // Set up dear imgui
    let mut imgui = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );
    imgui.set_ini_filename(None);

    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    //
    // Set up dear imgui wgpu renderer
    //
    let clear_color = wgpu::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };
    let mut renderer = Renderer::new(&mut imgui, &mut device, sc_desc.format, Some(clear_color));

    let mut state = State {
        viewport_pos: [0.0, 0.0],
    };

    // Event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                size = window.inner_size().to_physical(hidpi_factor);

                sc_desc = wgpu::SwapChainDescriptor {
                    usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    width: size.width as u32,
                    height: size.height as u32,
                    present_mode: wgpu::PresentMode::NoVsync,
                };

                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            }
            | Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::EventsCleared => {
                let frame = swap_chain.get_next_texture();
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let ui = imgui.frame();
                draw_ui(&mut state, &ui);

                let mut encoder: wgpu::CommandEncoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

                platform.prepare_render(&ui, &window);
                renderer
                    .render(ui, &mut device, &mut encoder, &frame.view)
                    .expect("Rendering failed");

                device.get_queue().submit(&[encoder.finish()]);

                std::thread::sleep(std::time::Duration::from_millis(4));
            }
            _ => (),
        }

        platform.handle_event(imgui.io_mut(), &window, &event);
    });
}

fn draw_ui(state: &mut State, ui: &Ui<'_>) {
    imgui::Window::new(im_str!("Node Editor Container Window")).build(ui, || {
        node_editor::NodeEditor::new(im_str!("Node Editor")).build(ui, &mut state.viewport_pos);
    });
}

struct State {
    viewport_pos: [f32; 2],
}

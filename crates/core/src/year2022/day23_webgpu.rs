use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    const MAX_SIZE: usize = 256;

    let gpu = crate::common::gpu::setup()?;

    let is_outside_max = |position: (i16, i16)| {
        position.0 < 0
            || position.1 < 0
            || position.0 >= MAX_SIZE as i16
            || position.1 >= MAX_SIZE as i16
    };

    let mut elf_data = vec![0_u32; MAX_SIZE * MAX_SIZE];

    let mut num_elves = 0;

    for (x, y) in input.text.lines().enumerate().flat_map(|(y, line)| {
        line.bytes()
            .enumerate()
            .filter_map(move |(x, c)| (c == b'#').then_some((x as i16 + 72, y as i16 + 24)))
    }) {
        if is_outside_max((x, y)) {
            return Err(format!("Elf is outside of [0,{MAX_SIZE})"));
        }
        elf_data[x as usize + y as usize * MAX_SIZE] = 1;
        num_elves += 1;
    }

    let buffer_usages = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let gpu_elf_buffer_0 = gpu.new_buffer_from(&elf_data, buffer_usages);
    let gpu_elf_buffer_1 = gpu.new_buffer_sized_as(&gpu_elf_buffer_0, buffer_usages);

    let size_buffer = gpu.new_buffer_from(&[MAX_SIZE as u32], wgpu::BufferUsages::UNIFORM);
    let mut rule_bits = 0b1001_0100_0010_1001_1110_0000_0000_0111_u32;
    let rule_buffer = gpu.new_buffer_from(
        &[rule_bits],
        wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    );

    let orig_moved_buffer = gpu.new_buffer_from(
        &[0_u32],
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    );
    let moved_buffer = gpu.new_buffer_from(
        &[0_u32],
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
    );
    let moved_staging_buffer = gpu.new_buffer_from(
        &[0_u32],
        wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
    );

    let bind_group_layout: wgpu::BindGroupLayout =
        gpu.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

    let create_bind_group = |from_buffer, to_buffer, bind_group_name| {
        let entries = [
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: from_buffer,
                    offset: 0,
                    size: None,
                }),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: to_buffer,
                    offset: 0,
                    size: None,
                }),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &size_buffer,
                    offset: 0,
                    size: None,
                }),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &moved_buffer,
                    offset: 0,
                    size: None,
                }),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &rule_buffer,
                    offset: 0,
                    size: None,
                }),
            },
        ];
        let descriptor = wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            label: Some(bind_group_name),
            entries: &entries,
        };
        gpu.device.create_bind_group(&descriptor)
    };

    let compute_bind_group_from_0_to_1 =
        create_bind_group(&gpu_elf_buffer_0, &gpu_elf_buffer_1, "compute_bind_group_0");
    let compute_bind_group_from_1_to_0 =
        create_bind_group(&gpu_elf_buffer_1, &gpu_elf_buffer_0, "compute_bind_group_1");

    let shader = gpu
        .device
        .create_shader_module(wgpu::include_wgsl!("day23_webgpu.wgsl"));

    let compute_pipeline_layout =
        gpu.device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

    let propose_pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            module: &shader,
            entry_point: Some("propose_movement"),
            compilation_options: Default::default(),
            cache: None,
        });

    let move_pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            module: &shader,
            entry_point: Some("apply_movement"),
            compilation_options: Default::default(),
            cache: None,
        });

    for round in 0..input.part_values(10, 1000) {
        let mut command_encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        command_encoder.copy_buffer_to_buffer(
            &orig_moved_buffer,
            0,
            &moved_buffer,
            0,
            moved_buffer.size(),
        );

        let mut pass_encoder =
            command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        pass_encoder.set_pipeline(&propose_pipeline);
        pass_encoder.set_bind_group(0, &compute_bind_group_from_0_to_1, &[]);
        let workgroup_width = 8;
        let workgroup_count_x = MAX_SIZE as u32 / workgroup_width;
        let workgroup_count_y = workgroup_count_x;
        let workgroup_count_z = 1;
        pass_encoder.dispatch_workgroups(workgroup_count_x, workgroup_count_y, workgroup_count_z);
        drop(pass_encoder);

        let mut pass_encoder =
            command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        pass_encoder.set_pipeline(&move_pipeline);
        pass_encoder.set_bind_group(0, &compute_bind_group_from_1_to_0, &[]);
        let workgroup_width = 8;
        let workgroup_count_x = MAX_SIZE as u32 / workgroup_width;
        let workgroup_count_y = workgroup_count_x;
        let workgroup_count_z = 1;
        pass_encoder.dispatch_workgroups(workgroup_count_x, workgroup_count_y, workgroup_count_z);
        drop(pass_encoder);

        command_encoder.copy_buffer_to_buffer(
            &moved_buffer,
            0,
            &moved_staging_buffer,
            0,
            moved_buffer.size(),
        );

        gpu.queue
            .write_buffer(&rule_buffer, 0, bytemuck::cast_slice(&[rule_bits]));
        rule_bits = rule_bits.rotate_right(8);

        gpu.queue.submit(std::iter::once(command_encoder.finish()));

        if input.is_part_two() {
            let moved_staging_buffer_slice = moved_staging_buffer.slice(..);
            moved_staging_buffer_slice.map_async(wgpu::MapMode::Read, Result::unwrap);
            gpu.instance.poll_all(true);
            let r = moved_staging_buffer_slice.get_mapped_range();
            let moved_data: &[u32] = bytemuck::cast_slice(&r);
            if moved_data[0] == 0 {
                return Ok(round + 1);
            }
            drop(r);
            moved_staging_buffer.unmap();
        }
    }

    if input.is_part_one() {
        let mut command_encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let debug_buffer = gpu.new_buffer_sized_as(
            &gpu_elf_buffer_0,
            wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        );

        command_encoder.copy_buffer_to_buffer(
            &gpu_elf_buffer_0,
            0,
            &debug_buffer,
            0,
            gpu_elf_buffer_0.size(),
        );
        gpu.queue.submit(std::iter::once(command_encoder.finish()));
        let debug_slice: wgpu::BufferSlice<'_> = debug_buffer.slice(..);
        debug_slice.map_async(wgpu::MapMode::Read, Result::unwrap);
        gpu.instance.poll_all(true);
        let r = debug_slice.get_mapped_range();
        let debug_data: &[u32] = bytemuck::cast_slice(&r);
        let (min_x, max_x, min_y, max_y) = debug_data.iter().enumerate().fold(
            (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
            |acc, (idx, cell)| {
                if *cell > 0 {
                    let x = idx % MAX_SIZE;
                    let y = idx / MAX_SIZE;
                    (acc.0.min(x), acc.1.max(x), acc.2.min(y), acc.3.max(y))
                } else {
                    acc
                }
            },
        );
        drop(r);
        debug_buffer.unmap();
        let rectangle_size = (max_x + 1 - min_x) * (max_y + 1 - min_y);
        return Ok(rectangle_size - num_elves);
    }

    Err("No solution found".to_string())
}

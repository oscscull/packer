import init, { init_shape_packer } from './pkg/shape_packer.js';

async function run() {
    await init();

    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');

    const shapePacker = init_shape_packer(canvas.width, canvas.height);
    
    // Example: Add some shapes
    shapePacker.add_shape(100, 100, 50);
    shapePacker.add_shape(200, 200, 30);
    shapePacker.add_shape(300, 300, 40);

    // Pack shapes (you'll implement this later)
    shapePacker.pack_shapes();

    // Draw shapes
    shapePacker.draw(ctx);
}

run();
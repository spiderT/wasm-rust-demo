let canvas, canvas2, ctx, ctx2, newimg, img2;

import("../crate/pkg").then((module) => {
  let startTime;
  let endTime;

  setUpEventListeners();

  function setUpEventListeners() {
    document
      .getElementById("img_uploader")
      .addEventListener("change", readURL, true);

    let effect_buttons = document.getElementsByClassName("effect");
    for (let i = 0; i < effect_buttons.length; i++) {
      let button = effect_buttons[i];
      button.addEventListener(
        "click",
        function () {
          applyEffect(event);
        },
        false
      );
    }

    let filter_buttons = document.getElementsByClassName("filter");
    for (let i = 0; i < filter_buttons.length; i++) {
      let button = filter_buttons[i];
      button.addEventListener(
        "click",
        function () {
          filterImage(event);
        },
        false
      );
    }

    setUpImages();
  }

  function applyEffect(event) {
    ctx.drawImage(newimg, 0, 0);
    startTime = performance.now();

    let filter_name = event.target.id;

    let rust_image = module.open_image(canvas, ctx);

    let filter_dict = {
      inc_red_channel: function () {
        return module.alter_channel(rust_image, 0, 90);
      },
      inc_blue_channel: function () {
        return module.alter_channel(rust_image, 2, 90);
      },
      inc_green_channel: function () {
        return module.alter_channel(rust_image, 1, 90);
      },
      inc_two_channels: function () {
        return module.alter_channel(rust_image, 1, 30);
      },
      dec_red_channel: function () {
        return module.alter_channel(rust_image, 0, -30);
      },
      dec_blue_channel: function () {
        return module.alter_channel(rust_image, 2, -30);
      },
      dec_green_channel: function () {
        return module.alter_channel(rust_image, 1, -30);
      },
      swap_rg_channels: function () {
        return module.swap_channels(rust_image, 0, 1);
      },
      swap_rb_channels: function () {
        return module.swap_channels(rust_image, 0, 2);
      },
      swap_gb_channels: function () {
        return module.swap_channels(rust_image, 1, 2);
      },
      remove_red_channel: function () {
        return module.remove_red_channel(rust_image, 250);
      },
      remove_green_channel: function () {
        return module.remove_green_channel(rust_image, 250);
      },
      remove_blue_channel: function () {
        return module.remove_blue_channel(rust_image, 250);
      },
      hue_rotate_hsl: function () {
        module.hue_rotate_hsl(rust_image, 0.3);
      },
      hue_rotate_hsv: function () {
        module.hue_rotate_hsv(rust_image, 0.3);
      },
      hue_rotate_lch: function () {
        module.hue_rotate_lch(rust_image, 0.3);
      },
      lighten_hsl: function () {
        module.lighten_hsl(rust_image, 0.3);
      },
      lighten_hsv: function () {
        module.lighten_hsv(rust_image, 0.3);
      },
      lighten_lch: function () {
        module.lighten_lch(rust_image, 0.3);
      },
      darken_hsl: function () {
        module.darken_hsl(rust_image, 0.3);
      },
      darken_hsv: function () {
        module.darken_hsv(rust_image, 0.3);
      },
      darken_lch: function () {
        module.darken_lch(rust_image, 0.3);
      },
      desaturate_hsl: function () {
        module.desaturate_hsl(rust_image, 0.3);
      },
      desaturate_hsv: function () {
        module.desaturate_hsv(rust_image, 0.3);
      },
      desaturate_lch: function () {
        module.desaturate_lch(rust_image, 0.3);
      },
      saturate_hsl: function () {
        module.saturate_hsl(rust_image, 0.3);
      },
      saturate_hsv: function () {
        module.saturate_hsv(rust_image, 0.3);
      },
      saturate_lch: function () {
        module.saturate_lch(rust_image, 0.3);
      },
    };

    filter_dict[filter_name]();
    module.putImageData(canvas, ctx, rust_image);

    endTime = performance.now();
    updateBenchmarks();
  }

  function filterImage(event) {
    startTime = performance.now();
    ctx.drawImage(newimg, 0, 0);
    let filter_name = event.target.id;

    console.time("wasm_time");

    let rust_image = module.open_image(canvas, ctx);

    module.filter(rust_image, filter_name);

    module.putImageData(canvas, ctx, rust_image);

    endTime = performance.now();
    updateBenchmarks();
    console.timeEnd("wasm_time");
  }

  function updateBenchmarks() {
    console.log("update benchmarks");
    let time_taken = endTime - startTime;
    let time_elem = document.getElementById("time");
    time_elem.innerHTML = `Time: ${time_taken.toFixed(2)}ms`;
  }

  function readURL() {
    let file = document.getElementById("img_uploader").files[0];

    let reader = new FileReader();
    reader.onloadend = function () {
      newimg.src = reader.result;
    };
    if (file) {
      reader.readAsDataURL(file);
    } else {
      console.log("Could not read file. :(");
    }
  }

  function setUpCanvas() {
    let element = document.getElementById("image_container");
    element.appendChild(newimg);

    canvas = document.getElementById("canvas");
    canvas.width = newimg.width;
    canvas.height = newimg.height;

    ctx = canvas.getContext("2d");
    ctx.drawImage(newimg, 0, 0);
  }

  function setUpCanvas2() {
    let element = document.getElementById("image_container");
    element.appendChild(img2);
    canvas2 = document.createElement("canvas");
    canvas2.width = img2.width;
    canvas2.height = img2.width;

    ctx2 = canvas2.getContext("2d");
    ctx2.drawImage(img2, 0, 0);
  }

  function setUpImages() {
    newimg = new Image();
    newimg.onload = () => {
      setUpCanvas();
    };

    img2 = new Image();
    img2.style.display = "none";
    img2.onload = () => {
      setUpCanvas2();
    };
    let change_image_elems = document.getElementsByClassName("change_image");

    for (let i = 0; i < change_image_elems.length; i++) {
      let change_image_elem = change_image_elems[i];
      change_image_elem.addEventListener(
        "click",
        function (event) {
          console.log("image changed");
          let img_name = event.target.id;
          let imgNamesToImages = {};
          newimg.src = imgNamesToImages[img_name];
          newimg.onload = () => {
            canvas.width = newimg.width;
            canvas.height = newimg.height;
            ctx.drawImage(newimg, 0, 0);
          };
        },
        false
      );
    }
  }
});

function editImage(canvas, ctx) {
  let imgData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  for (let i = 0; i < imgData.data.length; i += 4) {
    imgData[i] += 30;
  }
  ctx.putImageData(imgData, 0, 0);
}

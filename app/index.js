const infuser = import("./infuse.js");

infuser.then(infuser => {
  infuser.init();

  function processFile(file) {
    console.info('[processing] file name = ' + file.name);

    var reader = new FileReader();

    reader.onload = function(ev) {
      console.info('[processing] reading "' + file.name + '" ended');
      var view = new Uint8Array(ev.target.result);
      infuser.process_file_data(view);
    }

    reader.readAsArrayBuffer(file);
  }

  window.dropHandler = function (ev) {
    console.debug('[drag] file(s) dropped');

    // Prevent default behavior (Prevent file from being opened)
    ev.preventDefault();

    if (ev.dataTransfer.items) {
      console.debug('[drag] ' + ev.dataTransfer.items.length + ' files');
      // Use DataTransferItemList interface to access the file(s)
      for (var i = 0; i < ev.dataTransfer.items.length; i++) {
        // If dropped items aren't files, reject them
        if (ev.dataTransfer.items[i].kind === 'file') {
          var file = ev.dataTransfer.items[i].getAsFile();
          console.debug('[drag] processing file[' + i + ']');
          processFile(file);
        }
      }
    } else {
      console.debug('[drag] ' + ev.dataTransfer.files.length + ' files');
      // Use DataTransfer interface to access the file(s)
      for (var i = 0; i < ev.dataTransfer.files.length; i++) {
        console.debug('[drag] processing file[' + i + ']');
        processFile(ev.dataTransfer.files[i]);
      }
    }

    // Pass event to removeDragData for cleanup
    removeDragData(ev)
  }

  window.dragOverHandler = function (ev) {
    console.debug('[drag] file(s) in drop zone');

    // Prevent default behavior (Prevent file from being opened)
    ev.preventDefault();
  }

  window.removeDragData = function (ev) {
    console.debug('[drag] removing drag data')

    if (ev.dataTransfer.items) {
      // Use DataTransferItemList interface to remove the drag data
      ev.dataTransfer.items.clear();
    } else {
      // Use DataTransfer interface to remove the drag data
      ev.dataTransfer.clearData();
    }
  }
});

;; a simple major mode for viewing wasm files as text format

(add-to-list 'file-name-handler-alist '("\\.wasm$" . wasm-handler))

(defun wasm-handler (op &rest args)
  "Handle .wasm files by putting the output of wasm2wat in the buffer."
  (cond
   ((eq op 'get-file-buffer)
    (let ((file (car args)))
      (with-current-buffer (create-file-buffer file)
        (call-process "wasm2wat" nil (current-buffer) nil
                      file)
        (setq buffer-file-name file)
        (setq buffer-read-only t)
        (set-buffer-modified-p nil)
        (goto-char (point-min))
        (fundamental-mode)
        (current-buffer))))
   ((wasm-handler-real op args))))

(defun wasm-handler-real (operation args)
  "Run the real handler without the wasm handler installed."
  (let ((inhibit-file-name-handlers
         (cons 'wasm-handler
               (and (eq inhibit-file-name-operation operation)
                    inhibit-file-name-handlers)))
        (inhibit-file-name-operation operation))
    (apply operation args)))

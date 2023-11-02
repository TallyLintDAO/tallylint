import { Dialog } from "quasar"

interface ConfirmConfig {
  title: string
  message: string
  okMethod: (data?: any) => void
  cancelMethod?: () => void
}

export const confirmDialog = (config: ConfirmConfig) => {
  const { title, message, okMethod, cancelMethod } = config
  return Dialog.create({
    title: title,
    message: message,
    cancel: true,
  })
    .onOk(() => {
      if (okMethod) okMethod()
    })
    .onCancel(() => {
      if (cancelMethod) cancelMethod()
    })
    .onDismiss(() => {
      // console.log('I am triggered on both OK and Cancel')
    })
}

export const inputDialog = (config: ConfirmConfig) => {
  const { title, message, okMethod, cancelMethod } = config
  return Dialog.create({
    title: title,
    message: message,
    cancel: true,
    persistent: true,
    prompt: {
      model: "",
      isValid: (val) => val.length > 0, // << here is the magic
      type: "text", // optional
    },
  })
    .onOk((data) => {
      if (okMethod) okMethod(data)
    })
    .onCancel(() => {
      if (cancelMethod) cancelMethod()
    })
    .onDismiss(() => {
      // console.log('I am triggered on both OK and Cancel')
    })
}

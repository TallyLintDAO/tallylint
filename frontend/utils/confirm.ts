import { Dialog } from 'quasar'

interface ConfirmConfig {
    title: string;
    message: string;
    okMethod: () => void;
    cancelMethod?: () => void
}

export const confirmDialog = (config: ConfirmConfig) => {
    const {title, message, okMethod, cancelMethod} = config;
    return Dialog.create({
        title: title,
        message: message,
        cancel: true,
    }).onOk(() => {
        if (okMethod) okMethod()
    }).onCancel(() => {
        if (cancelMethod) cancelMethod();
    }).onDismiss(() => {
        // console.log('I am triggered on both OK and Cancel')
    })
}



export type SvelteComponent = import('svelte').ComponentType;
export type SvelteToastOnPopCallback = (id?: number | undefined) => any;

export type SvelteToastCustomComponent = {
    /**
     * - custom Svelte Component
     */
    src: SvelteComponent;
    /**
     * - props to pass into custom component
     */
    props?: {
        [x: string]: any;
    } | undefined;
    /**
     * - forward toast id to prop name
     */
    sendIdTo?: string | undefined;
};

export type SvelteToastOptions = {
    /**
     * - unique id generated for every toast
     */
    id?: number | undefined;
    /**
     * - container target name to send toast to
     */
    target?: string | undefined;
    /**
     * - toast message
     */
    msg?: string | undefined;
    /**
     * - duration of progress bar tween from initial to next
     */
    duration?: number | undefined;
    /**
     * - initial progress bar value
     */
    initial?: number | undefined;
    /**
     * - next progress bar value
     */
    next?: number | undefined;
    /**
     * - pause progress bar tween on mouse hover
     */
    pausable?: boolean | undefined;
    /**
     * - allow dissmiss with close button
     */
    dismissable?: boolean | undefined;
    /**
     * - display toasts in reverse order
     */
    reversed?: boolean | undefined;
    /**
     * - toast intro fly animation settings
     */
    intro?: import("svelte/transition").FlyParams | undefined;
    /**
     * - css var overrides
     */
    theme?: {
        [x: string]: string | number;
    } | undefined;
    /**
     * - user-defined classes
     */
    classes?: string[] | undefined;
    /**
     * - callback that runs on toast dismiss
     */
    onpop?: SvelteToastOnPopCallback | undefined;
    /**
     * - send custom Svelte Component as a message
     */
    component?: SvelteToastCustomComponent | undefined;
    /**
     * - DEPRECATED
     */
    progress?: number | undefined;
};

export const toastTheme: Record<"success" | "error", SvelteToastOptions> = {
    success: {
        theme: {
            '--toastColor': 'mintcream',
            '--toastBackground': 'rgba(72,187,120,0.9)',
            '--toastBarBackground': '#2F855A'
        },
        pausable: true,
    },
    error: {
        theme: {
            '--toastColor': 'white',
            '--toastBackground': 'rgba(239,68,68,0.9)',
            '--toastBarBackground': '#E53E3E'
        },
        pausable: true,
    }
}
<script lang="ts">
  import { createClass } from "./utils/createClass";

  export let variant: "primary" | "secondary" | "danger" | "success" =
    "primary";

  export let size: "sm" | "md" | "lg" = "sm";

  let variants = {
    disabled: {
      primary: "bg-blue-400 cursor-not-allowed",
      secondary: "bg-gray-400 cursor-not-allowed",
      danger: "bg-red-400 cursor-not-allowed",
      success: "bg-green-400 cursor-not-allowed",
    },
    enabled: {
      primary: "bg-blue-600 hover:bg-blue-700",
      secondary: "bg-gray-600 hover:bg-gray-700",
      danger: "bg-red-600 hover:bg-red-700",
      success: "bg-green-700 hover:bg-green-800",
    },
  };

  const sizes = {
    sm: "px-2 py-1 text-sm",
    md: "px-3 py-2 text-md",
    lg: "px-4 py-3 text-lg",
  };

  const customClass = $$props.class;

  $: btnClass = createClass(
    variants[$$props.disabled ? "disabled" : "enabled"][variant],
    sizes[size],
    "text-white rounded-md",
    customClass
  );
</script>

<button on:click|preventDefault {...$$restProps} class={btnClass}>
  <slot />
</button>

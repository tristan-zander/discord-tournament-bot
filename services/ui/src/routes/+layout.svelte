<script>
    import "../app.css";
    import SideDrawer from "$lib/drawer.svelte"
    import Footer from '$lib/footer.svelte'
</script>

<SideDrawer />

<div id="main-content-window">
    <slot />
</div>

<Footer />

<style lang="postcss">
    #main-content-window {
        grid-area: content;
        @apply overflow-y-scroll h-full w-full p-3 m-0 shadow-inner bg-slate-100;
    }

    :global(body) {
        grid-template:
            "drawer content content" 1fr
            "drawer footer footer" auto /
            auto 1fr 1fr;
        @apply grid h-screen w-screen overflow-hidden;
    }
</style>
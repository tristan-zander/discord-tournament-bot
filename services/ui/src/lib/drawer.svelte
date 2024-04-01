
<!-- Represents the toolbar that sits at the top or left of the page for naviagation -->
<ul id="side-drawer">
    <li><img class="icon" src="/menu.svg" alt="Expand drawer"></li>
    <li class="space-y-10 flex-grow"></li>
    <li><img class="icon" src="/dashboard.svg" alt="Go to Dashboard"></li>
    <li><img class="icon" src="/trophy.svg" alt="Go to Tournaments"></li>
    <li><img class="icon" src="/console.svg" alt="Go to Matchmaking"></li>
</ul>

<style lang="postcss">
    #side-drawer {
        @apply flex flex-col h-screen shadow-lg p-1 gap-5;
        grid-area: drawer;
    }

    .icon {
        @apply h-6 w-6;
    }
</style>

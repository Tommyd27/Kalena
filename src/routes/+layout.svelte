<script>
  import "../app.postcss";
  import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Dropdown, DropdownItem, Chevron, DropdownDivider } from 'flowbite-svelte'
</script>

<main class="container">
  <Navbar let:hidden let:toggle>
    <NavBrand href="/">
      <img
        src="/KalenaIcon.png"
        class="mr-3 h-6 sm:h-9"
        alt="Kalena Icon"
      />
      <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
        Kalena
      </span>
    </NavBrand>
    <NavHamburger on:click={toggle} />
    <NavUl {hidden}>
      <NavLi id="nav-menu1" class="cursor-pointer"><Chevron aligned>Gaming</Chevron></NavLi>
      <Dropdown triggeredBy="#nav-menu1" class="w-44 z-20">
        <DropdownItem href="/rocket-league-players">Rocket League Players</DropdownItem>
      </Dropdown>
    </NavUl>
  </Navbar>
  <slot />
</main>

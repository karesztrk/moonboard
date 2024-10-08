<script lang="ts">
  const year = new Date().getFullYear();
</script>

<header>
  <hgroup>
    <h1>Moonboard</h1>
    <p>A desktop app to play with my Moonlander</p>
  </hgroup>

  <nav>
    <ul>
      <li><a href="/practice">Practice</a></li>
      <li><a href="/animation">Animation</a></li>
    </ul>
  </nav>
</header>
<main>
  <slot />
</main>
<footer>
  <small>Károly Török &amp; Nimród Török &copy; {year}</small>
</footer>

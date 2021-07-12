<div class="banner">
  <img src="banner.jpg" alt="Banner">
  <h1 class="">Christian Payne</h1>
</div>

<style>
  :root {
    --hover-color: #F9FBFF;
  }
  
  @font-face {
    font-family: cursive;
    src: Pacifico-Regular;
  }

  .banner {
    position: relative;
  }
  
  .banner h1{
    /* Positioning */
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    
    /* Style */
    font-family: 'Pacifico', cursive;
    color: #E8EAED;
    text-shadow: -3px 5px 1px rgba(0, 0, 0, 0.3);

    /* Border */
    padding: 0.5em;
    border: #E8EAED solid 1px;
    border-radius: 0.5em;
    box-shadow: -1px 3px 1px rgba(0, 0, 0, 0.15);

    /* Utility */
    transition: color 0.3s, font-size 0.3s, border 0.3s;
    transition-timing-function: cubic-bezier(0.1, 0.5, 0.4, 1.6);
    -webkit-user-select: none;
  }
  .banner img {
    width: 100%;
    object-fit: none;
    object-position: 70% 75%;
  }

  @media screen and (min-width: 1025px) {
    .banner img {
      object-fit: fill;
    }
    .banner h1 {
      font-size: 2.5rem;
    }
    .banner h1:hover {
    color: var(--hover-color);
    font-size: 3em;
    border: var(--hover-color) solid 1px;
  }
  }
  @media screen and (min-width: 2000px) {
    .banner h1 {
      font-size: 4rem;
    }
    .banner h1:hover {
      color: var(--hover-color);
      font-size: 4.5em;
      border: var(--hover-color) solid 1px;
    }
  }
  @media screen and (max-width: 600px) {
    .banner h1 {
      font-size: 1.5rem;
      top: 30%;
      left: 35%;
    }
    .banner h1:hover {
      color: var(--hover-color);
      border: var(--hover-color) solid 1px;
    }
  }
</style>
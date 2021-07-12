<div class="banner">
  <img src="banner.jpg" alt="Banner">
  <h1 class="">Christian Payne</h1>
</div>

<style lang="scss">
  @import "../breakpoints.scss";
  :root {
    --hover-color: #F9FBFF;
  }
  
  @font-face {
    font-family: cursive;
    src: Pacifico-Regular;
  }

  .banner {
    position: relative;

    h1 {
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

      &:hover {
        color: var(--hover-color);
        border: var(--hover-color) solid 1px;
      }

      @include screen-sm-only {
        font-size: 1.5rem;
        top: 30%;
        left: 35%;
      }

      @include screen-lg-up {
        font-size: 2.5rem;

        &:hover {
          color: var(--hover-color);
          font-size: 3em;
          border: var(--hover-color) solid 1px;
        }
      }

      @include screen-2xl-up {
        font-size: 4rem;

        &:hover {
          color: var(--hover-color);
          font-size: 4.5em;
          border: var(--hover-color) solid 1px;
        }
      }
    }
    img {
      width: 100%;
      object-fit: none;
      object-position: 70% 75%;

      @include screen-lg-up {
        object-fit: fill;
      }
    }

  }
</style>
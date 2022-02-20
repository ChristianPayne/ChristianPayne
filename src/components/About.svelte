<div class="about">
  <img src="Headshot_1.png" alt="AboutMePic">
  <div class="styled-font">
    <i>"The trajectory of your life bends in the direction of your habits." - James Clear</i>
    <!-- <h2>About Me</h2>
    <p>
      
    </p> -->
    <h2>Full Stack Web Developer</h2>
    <p>
      Based in Southern California, currently working as a Software Developer at <a href="https://modelmatch.com/">Model Match</a>. I started out in Full Stack when the pandemic hit and shortly after, I enrolled online in UC Irvine's Web Dev bootcamp. Six fast-paced months later, I was well equipped to start a new job and have not looked back since.
    </p>
    <h2>Interactive Developer</h2>
    <p>
      I have a passion for creating interactive experiences. There is a game-like nature of interactive experiences that are very enticing to me. I started using Unity to develop interactives for autoshows back in 2016. That was my first time ever coding professionally. 
    </p>
    <h2>Tinkerer</h2>
    <p>
      Another interest of mine is microcontrollers! I have been dabbling in hardware electronics from a young age. Soldering circuits and getting them to work brings my imagination and code into a tangable space. The most interesting part of microcontrollers, in my opinion, is getting them to communicate with each other wirelessly and over the internet. Combining this hobby with my main knowledge base of software development, I have been able to improve on my simple projects.
    </p>
    <h2>Streamer</h2>
    <p>
      I have been streaming since 2015 off and on. While I started streaming games back in the day, I found out that coding on streams was more in demand than I expected. Typically, I stream coding to an audience who is not as tech-savvy as I am, generally. Because of that, I need to break down what I am doing into simple english terms. This process helps me reinforce myself and my work.
    </p>
    <!-- <h2>Teacher</h2>
    <p>
      I do the stuff and things to teach kids
    </p> -->
  </div>
</div>

<style lang="scss">
  @import "../breakpoints.scss";

  .about {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-gap: 1.5em;

    img {
      border-radius: 0.5em;
      width: 100%;
    }

    i {
      font-size: 1.5em;
    }

    @include screen-sm-only {
      grid-template-columns: 1fr;
    }

    @include screen-md-up {
      grid-template-columns: 1fr 1fr;
    }

    @include screen-lg-up {
      grid-template-columns: 1fr 2fr;
    }

    @include screen-xl-up {
      grid-template-columns: 1fr 3fr;
    }

    @include screen-2xl-up {
      grid-template-columns: 1fr 4fr;
    }
  }
</style>
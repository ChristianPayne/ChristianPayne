<script>
  let technologies = [
    { name: 'React', category: "Frameworks", icon: "devicon-react-original"},
    { name: 'Angular', category: "Frameworks", icon: "devicon-angularjs-plain"},
    { name: 'Vue', category: "Frameworks", icon: "devicon-vuejs-plain"},
    { name: 'Svelte', category: "Frameworks", icon: "devicon-svelte-plain"},
    { name: 'Nextjs', category: "Frameworks", icon: "devicon-nextjs-original"},
    { name: 'Electron', category: "Frameworks", icon: "devicon-electron-original"},
    { name: 'Material UI', category: "Frameworks", icon: "devicon-materialui-plain"},
    { name: 'Tailwind CSS', category: "Frameworks", icon: "devicon-tailwindcss-original-wordmark"},
    { name: 'Bootstrap', category: "Frameworks", icon: "devicon-bootstrap-plain"},
    { name: 'JQuery', category: "Frameworks", icon: "devicon-jquery-plain"},
    { name: 'Handlebars', category: "Frameworks", icon: "devicon-handlebars-plain"},
    { name: 'Flutter', category: "Frameworks", icon: "devicon-flutter-plain"},
    { name: 'TypeScript', category: "Languages", icon: "devicon-typescript-plain"},
    { name: 'JavaScript', category: "Languages", icon: "devicon-javascript-plain"},
    { name: 'C#', category: "Languages", icon: "devicon-csharp-plain"},
    { name: 'Python', category: "Languages", icon: "devicon-python-plain"},
    { name: 'HTML5', category: "Languages", icon: "devicon-html5-plain"},
    { name: 'CSS', category: "Languages", icon: "devicon-css3-plain"},
    { name: 'Sass', category: "Languages", icon: "devicon-sass-original"},
    { name: 'GraphQL', category: "Languages", icon: "devicon-graphql-plain"},
    { name: 'Markdown', category: "Languages", icon: "devicon-markdown-original"},
    { name: 'AWS', category: "Dev Ops", icon: "devicon-amazonwebservices-original"},
    { name: 'Netlify', category: "Dev Ops", icon: ""},
    { name: 'Heroku', category: "Dev Ops", icon: "devicon-heroku-original"},
    { name: 'Docker', category: "Dev Ops", icon: "devicon-docker-plain"},
    { name: 'Travis', category: "Dev Ops", icon: "devicon-travis-plain"},
    { name: 'SocketIO', category: "Dev Ops", icon: "devicon-socketio-original"},
    { name: 'Nodejs', category: "Dev Tools", icon: "devicon-nodejs-plain"},
    { name: 'Bash', category: "Dev Tools", icon: "devicon-bash-plain"},
    { name: 'Chrome', category: "Dev Tools", icon: "devicon-chrome-plain"},
    { name: 'SSH', category: "Dev Tools", icon: "devicon-ssh-original"},
    { name: 'VSCode', category: "Dev Tools", icon: "devicon-vscode-plain"},
    { name: 'Visual Studio', category: "Dev Tools", icon: "devicon-visualstudio-plain"},
    { name: 'Webpack', category: "Dev Tools", icon: "devicon-webpack-plain"},
    { name: 'Confluence', category: "Communication", icon: "devicon-confluence-original"},
    { name: 'Slack', category: "Communication", icon: "devicon-slack-plain"},
    { name: 'Trello', category: "Communication", icon: "devicon-trello-plain"},
    { name: 'Npm', category: "Version Control", icon: "devicon-npm-original-wordmark"},
    { name: 'Git', category: "Version Control", icon: "devicon-git-plain"},
    { name: 'GitHub', category: "Version Control", icon: "devicon-github-original"},
    { name: 'Bitbucket', category: "Version Control", icon: "devicon-bitbucket-original"},
    { name: 'Windows', category: "Hardware", icon: "devicon-windows8-original"},
    { name: 'Raspberry Pi', category: "Hardware", icon: "devicon-raspberrypi-line"},
    { name: 'Arduino', category: "Hardware", icon: "devicon-arduino-plain"},
    { name: 'WS2812B', category: "Hardware", icon: ""},
    { name: 'Unity', category: "Programs", icon: "devicon-unity-original"},
    { name: 'Figma', category: "Programs", icon: "devicon-figma-plain"},
    { name: 'Photoshop', category: "Programs", icon: "devicon-photoshop-plain"},
    { name: 'Premiere Pro', category: "Programs", icon: "devicon-premierepro-plain"},
    { name: 'Illustrator', category: "Programs", icon: "devicon-illustrator-plain"},
    { name: 'After Effects', category: "Programs", icon: "devicon-aftereffects-plain"},
    { name: 'Mongo DB', category: "Databases", icon: "devicon-mongodb-plain"},
    { name: 'Mongoose', category: "Databases", icon: "devicon-mongodb-plain"},
    { name: 'My SQL', category: "Databases", icon: "devicon-mysql-plain"},
    { name: 'Sequelize', category: "Databases", icon: "devicon-sequelize-plain"},
    { name: 'Fauna DB', category: "Databases", icon: ""},
    { name: 'FQL', category: "Databases", icon: ""},
  ];

  let sections = [...new Set(technologies.map(t => t.category))]

  function getTechnologies(name) {
    return technologies.filter(t => t.category === name);
  }

  // console.log(sections);
  // console.log(getTechnologies(sections[0]));

</script>

<div class="styled-font capabilities">
  {#each sections as section}
    <div class="technologies">
      <h2>{section}</h2>
      <div class="icons">
        {#each getTechnologies(section) as technology}
          {#if technology.icon}
            <i class={`${technology.icon} colored icon`}>
              <span class="font">
                {`${technology.name}`}
              </span>
            </i>
          {/if}
          {#if !technology.icon}
            <span class="icon">
              <img src="CPFav-02.png" alt="logo-icon" class="logo-icon">
              <span class="label font">{`${technology.name}`}</span>
            </span>
          {/if}
        {/each}
      </div>
    </div>
  {/each}
</div>

<style lang="scss">
  @import "../breakpoints.scss";
  .capabilities {
    display: grid;
    grid-gap: 1.5em;

    @include screen-sm-only {
      grid-template-columns: 1fr;
      grid-gap: 1em;
    }

    @include screen-md-up {
      grid-template-columns: 1fr 1fr;
    }

    @include screen-lg-up {
      grid-template-columns: 1fr 1fr 1fr;
    }

    .technologies {
      border: 1px solid #41444A;
      border-radius: 0.5em;
      padding: 1em;

      @include screen-sm-only {
        font-size: 0.8em;
      }

      h2 {
        margin: 0;
        margin-bottom: 0.5em;
        border-bottom: 1px solid #41444A;
      }

      .label {
        color: #202124
      }
      
    }
  }
  .icons {
    display: grid;
    grid-gap: .5em;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr))
  }

  .icon {
    font-size: 1.1em;
    font-weight: bold;
    padding: 0.4em;
    margin-right: 0.5em;
    background-color: #E8EAED;
    border-radius: 0.2em;
    cursor: default;
  }

  .logo-icon {
    max-width: 2.5em;
  }

  .icon span {
    margin-top: 1em;
    margin-left: 1em;
    word-wrap: break-word;
  }

  .icon:not(:hover) {
    color: #202124;
  }
</style>
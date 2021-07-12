<script>
  let technologies = [
    { name: 'React', category: "Frameworks", icon: "devicon-react-original"},
    { name: 'Angular', category: "Frameworks", icon: "devicon-angularjs-plain"},
    { name: 'Vue', category: "Frameworks", icon: "devicon-vuejs-plain"},
    { name: 'Nextjs', category: "Frameworks", icon: "devicon-nextjs-original"},
    { name: 'Electron', category: "Frameworks", icon: "devicon-electron-original"},
    { name: 'Material UI', category: "Frameworks", icon: "devicon-materialui-plain"},
    { name: 'Tailwind CSS', category: "Frameworks", icon: "devicon-tailwindcss-original-wordmark"},
    { name: 'Bootstrap', category: "Frameworks", icon: "devicon-bootstrap-plain"},
    { name: 'JQuery', category: "Frameworks", icon: "devicon-jquery-plain"},
    { name: 'Typescript', category: "Languages", icon: "devicon-typescript-plain"},
    { name: 'Javascript', category: "Languages", icon: "devicon-javascript-plain"},
    { name: 'C Sharp', category: "Languages", icon: "devicon-csharp-plain"},
    { name: 'Python', category: "Languages", icon: "devicon-python-plain"},
    { name: 'Html5', category: "Languages", icon: "devicon-html5-plain"},
    { name: 'CSS 3', category: "Languages", icon: "devicon-css3-plain"},
    { name: 'Sass', category: "Languages", icon: "devicon-sass-original"},
    { name: 'Handlebars', category: "Languages", icon: "devicon-handlebars-plain"},
    { name: 'Graphql', category: "Languages", icon: "devicon-graphql-plain"},
    { name: 'AWS', category: "Dev Ops", icon: "devicon-amazonwebservices-original"},
    { name: 'Heroku', category: "Dev Ops", icon: "devicon-heroku-original"},
    { name: 'Docker', category: "Dev Ops", icon: "devicon-docker-plain"},
    { name: 'Travis', category: "Dev Ops", icon: "devicon-travis-plain"},
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
    { name: 'Windows 8', category: "Hardware", icon: "devicon-windows8-original"},
    { name: 'Raspberry Pi', category: "Hardware", icon: "devicon-raspberrypi-line"},
    { name: 'Figma', category: "Content Creation", icon: "devicon-figma-plain"},
    { name: 'Photoshop', category: "Content Creation", icon: "devicon-photoshop-plain"},
    { name: 'Premiere Pro', category: "Content Creation", icon: "devicon-premierepro-plain"},
    { name: 'Illustrator', category: "Content Creation", icon: "devicon-illustrator-plain"},
    { name: 'After Effects', category: "Content Creation", icon: "devicon-aftereffects-plain"},
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

  console.log(sections);
  console.log(getTechnologies(sections[0]));

</script>

<div class="styled-font capabilities">
  {#each sections as section}
    <div class="technologies">
      <h2>{section}</h2>
      <div class="icons">
        {#each getTechnologies(section) as technology}
          <i class={`${technology.icon} colored`}><span class="label font">{`${technology.name}`}</span></i>
        {/each}
      </div>
    </div>
  {/each}
</div>

<style>
  .capabilities {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    grid-gap: 1.5em;
  }

  .capabilities .technologies {
    /* display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column; */
    border: 1px solid #41444A;
    border-radius: 0.5em;
    padding: 1em;
  }
  
  .capabilities .technologies h2 {
    margin: 0;
    margin-bottom: 0.5em;
    border-bottom: 1px solid #41444A;
  }

  .capabilities .technologies .icons {
    display: grid;
    grid-gap: .5em;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr))
  }

  .capabilities .technologies .icons i {
    font-size: 1.1em;
    font-weight: bold;
    padding: 0.4em;
    margin-right: 0.5em;
    background-color: #E8EAED;
    border-radius: 0.2em;
    cursor: default;
  }

  .capabilities .technologies .icons i span{
    margin-top: 1em;
    margin-left: 1em;
    word-wrap: break-word;
  }

  .capabilities .technologies i:not(:hover) {
    color: #202124;
  }

  @media screen and (max-width: 1025px) {
    .capabilities {
      grid-template-columns: 1fr 1fr;
    }
  }
  @media screen and (max-width: 600px) {
    .capabilities {
      display: grid;
      grid-template-columns: 1fr;
      grid-gap: 1.5em;
    }
    .capabilities .technologies {
      font-size: 0.8em;
    }
  }
</style>
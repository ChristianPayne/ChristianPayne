<script>
  let technologies = [
    { name: "react", category: "Frameworks", icon: "devicon-react-original"},
    { name: "angularjs", category: "Frameworks", icon: "devicon-angularjs-plain"},
    { name: "vuejs", category: "Frameworks", icon: "devicon-vuejs-plain"},
    { name: "nextjs", category: "Frameworks", icon: "devicon-nextjs-original"},
    { name: "electron", category: "Frameworks", icon: "devicon-electron-original"},
    { name: "materialui", category: "Frameworks", icon: "devicon-materialui-plain"},
    { name: "tailwindcss", category: "Frameworks", icon: "devicon-tailwindcss-original-wordmark"},
    { name: "bootstrap", category: "Frameworks", icon: "devicon-bootstrap-plain"},
    { name: "jquery", category: "Frameworks", icon: "devicon-jquery-plain"},
    { name: "typescript", category: "Languages", icon: "devicon-typescript-plain"},
    { name: "javascript", category: "Languages", icon: "devicon-javascript-plain"},
    { name: "csharp", category: "Languages", icon: "devicon-csharp-plain"},
    { name: "python", category: "Languages", icon: "devicon-python-plain"},
    { name: "html5", category: "Languages", icon: "devicon-html5-plain"},
    { name: "css3", category: "Languages", icon: "devicon-css3-plain"},
    { name: "sass", category: "Languages", icon: "devicon-sass-original"},
    { name: "handlebars", category: "Languages", icon: "devicon-handlebars-plain"},
    { name: "graphql", category: "Languages", icon: "devicon-graphql-plain"},
    { name: "amazonwebservices", category: "Dev Ops", icon: "devicon-amazonwebservices-original"},
    { name: "confluence", category: "Dev Ops", icon: "devicon-confluence-original"},
    { name: "heroku", category: "Dev Ops", icon: "devicon-heroku-original"},
    { name: "docker", category: "Dev Ops", icon: "devicon-docker-plain"},
    { name: "travis", category: "Dev Ops", icon: "devicon-travis-plain"},
    { name: "nodejs", category: "Dev Tools", icon: "devicon-nodejs-plain"},
    { name: "bash", category: "Dev Tools", icon: "devicon-bash-plain"},
    { name: "chrome", category: "Dev Tools", icon: "devicon-chrome-plain"},
    { name: "ssh", category: "Dev Tools", icon: "devicon-ssh-original"},
    { name: "vscode", category: "Dev Tools", icon: "devicon-vscode-plain"},
    { name: "visualstudio", category: "Dev Tools", icon: "devicon-visualstudio-plain"},
    { name: "webpack", category: "Dev Tools", icon: "devicon-webpack-plain"},
    { name: "slack", category: "Communication", icon: "devicon-slack-plain"},
    { name: "trello", category: "Communication", icon: "devicon-trello-plain"},
    { name: "npm", category: "Version Control", icon: "devicon-npm-original-wordmark"},
    { name: "git", category: "Version Control", icon: "devicon-git-plain"},
    { name: "github", category: "Version Control", icon: "devicon-github-original"},
    { name: "bitbucket", category: "Version Control", icon: "devicon-bitbucket-original"},
    { name: "windows8", category: "Hardware", icon: "devicon-windows8-original"},
    { name: "raspberrypi", category: "Hardware", icon: "devicon-raspberrypi-line"},
    { name: "figma", category: "Content Creation", icon: "devicon-figma-plain"},
    { name: "photoshop", category: "Content Creation", icon: "devicon-photoshop-plain"},
    { name: "premierepro", category: "Content Creation", icon: "devicon-premierepro-plain"},
    { name: "illustrator", category: "Content Creation", icon: "devicon-illustrator-plain"},
    { name: "aftereffects", category: "Content Creation", icon: "devicon-aftereffects-plain"},
    { name: "mongodb", category: "Databases", icon: "devicon-mongodb-plain"},
    { name: "mysql", category: "Databases", icon: "devicon-mysql-plain"},
    { name: "sequelize", category: "Databases", icon: "devicon-sequelize-plain"},
  ];

  let sections = [...new Set(technologies.map(t => t.category))]

  function getTechnologies(name) {
    return technologies.filter(t => t.category === name);
  }

  console.log(sections);
  console.log(getTechnologies(sections[0]));

</script>

<div class="font capabilities">
  {#each sections as section}
    <div class="technologies">
      <h2>{section}</h2>
      <div class="icons">
        {#each getTechnologies(section) as technology}
          <i class={`${technology.icon} colored`}></i>
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
    margin-top: 1em;
  }

  .capabilities .technologies i {
    padding: 0.25em;
    margin-right: 0.5em;
    background-color: #E8EAED;
    border-radius: 0.2em;
  }

  .capabilities .technologies i:not(:hover) {
    color: black;
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
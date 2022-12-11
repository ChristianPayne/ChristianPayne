<script>
  let classNames = "";
  export { classNames as class};

  const quotes = [
    {
      quote: 'The rent of success is due everyday.',
      author: 'Darren Hardy'
    },
    {
      quote: 'The trajectory of your life bends in the direction of your habits.',
      author: 'James Clear'
    },
    {
      quote: 'We\'ve got to have a dream if we are going to make a dream come true.',
      author: 'Walt Disney'
    },
    {
      quote: 'What the internet did for information, NFTs will do for transaction.',
      author: 'Gary Vee'
    },
    {
      quote: 'Every man has two lives, and the second starts when he realizes he has just one.',
      author: 'Confucius'
    },
    {
      quote: 'Adversity is the foundation of success.',
      author: 'Gary Vee'
    },
    {
      quote: 'Luck is what happens when preparation meets opportunity.',
      author: 'Seneca'
    },
    {
      quote: 'The fact that you won\'t have to ask yourself \'Why didn\'t I try?\' You\'ve already won!',
      author: 'Gary Vee'
    },
    {
      quote: 'Unique, expensive, sticky air.',
      author: 'Alex Hormozi'
    },
    {
      quote: 'Desire is a contract we make with ourselves to be unhappy until we get what we want.',
      author: 'Naval Ravikant'
    },
    {
      quote: 'We don\'t think ourselves into new ways of living, we live ourselves into new ways of thinking.',
      author: 'Richard Rohr'
    },
    {
      quote: 'Success is what you attract by the person you become.',
      author: 'Jim Rohn'
    },
    {
      quote: 'If you work hard on your job, you can make a living. If you work hard on your self, you can make a fortune.',
      author: 'Jim Rohn'
    },
    {
      quote: 'A problem difficult at night is resolved in the morning after the committee of sleep has worked on it.',
      author: 'John Steinbeck'
    },
    {
      quote: 'You never really get an opportunity, you take an opportunity.',
      author: 'James Cameron'
    },
  ]

  function getRandomQuote () {
    let randomIndex = Math.floor(Math.random() * quotes.length);
    return {quote: quotes[randomIndex], index: randomIndex};
  }

  let randomQuote = getRandomQuote();
</script>

<h2 class={classNames}><i>"{randomQuote.quote.quote}"</i> - {randomQuote.quote.author} {randomQuote.index + 1}/{quotes.length}</h2>

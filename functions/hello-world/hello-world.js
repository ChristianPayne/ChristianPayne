// Docs on event and context https://www.netlify.com/docs/functions/#the-handler-method
const handler = async (event) => {
  try {
    console.log("Hello World Triggered.");
    const subject = event.queryStringParameters.name || 'World'
    const returnValue = JSON.stringify({data: `Hello ${subject}`})
    console.log('Returning ',returnValue);
    return {
      statusCode: 200,
      body:returnValue,
      // // more keys you can return:
      // headers: { "headerName": "headerValue", ... },
      // isBase64Encoded: true,
    }
  } catch (error) {
    return { statusCode: 500, body: error.toString() }
  }
}

module.exports = { handler }

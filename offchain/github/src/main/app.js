/**
 * @param {import('probot').Probot} app
 */
module.exports = (app) => {
  app.log.info('Yay! The app was loaded!')

  // Listen for commands on issue comments
  app.on('issue_comment.created', async (context) => {
    const command = context.payload.comment.body.trim()
    const issue = context.payload.issue
    const isPullRequest = !!context.payload.issue.pull_request

    // Check if the comment starts with one of the specified commands
    if (command.startsWith('/bounty') && !isPullRequest) {
      // Handle bounty command
      const bountyAmount = command.split(' ')[1]
      const commentBody = `Creating bounty of $${bountyAmount} for issue #${issue.number}`
      await context.octokit.issues.createComment(context.issue({ body: commentBody }))
      app.log.info(commentBody)
    } else if (command.startsWith('/attempt') && !isPullRequest) {
      // Handle attempt command
      const attemptNumber = command.split(' ')[1]
      const commentBody = `Started working on attempt ${attemptNumber}`
      await context.octokit.issues.createComment(context.issue({ body: commentBody }))
      app.log.info(commentBody)
    }
  })

  // Listen for commands on PR comments
  app.on('pull_request_review_comment.created', async (context) => {
    const command = context.payload.comment.body.trim()

    // Check if the comment is the /approve command
    if (command === '/approve') {
      // Handle approve command
      const commentBody = 'Marking claim as ready to pay'
      await context.octokit.issues.createComment(context.issue({ body: commentBody }))
      app.log.info(commentBody)
    }
  })

  // Listen for commands on PR body
  app.on('pull_request.opened', async (context) => {
    const command = context.payload.pull_request.body.trim()

    // Check if the PR body contains the /claim command
    if (command.startsWith('/claim')) {
      // Handle claim command
      const issueNumber = command.split(' ')[1]
      const commentBody = `Submitting claim for bounty of issue ${issueNumber}`
      await context.octokit.issues.createComment(context.issue({ body: commentBody }))
      app.log.info(commentBody)
    }
  })

  // Log any event that the app receives
  app.onAny(async (context) => {
    app.log.info({ event: context.name, action: context.payload.action })
  })

  // Log errors
  app.onError(async (error) => {
    app.log.error(error)
  })
}

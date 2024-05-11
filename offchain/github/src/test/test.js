const { suite } = require('uvu')
const assert = require('uvu/assert')

const nock = require('nock')
nock.disableNetConnect()

const { Probot, ProbotOctokit } = require('probot')

const app = require('../main/app')

let probot
const test = suite('app')
test.before.each(() => {
  probot = new Probot({
    // simple authentication as alternative to appId/privateKey
    githubToken: 'test',
    // disable logs
    logLevel: 'warn',
    // disable request throttling and retries
    Octokit: ProbotOctokit.defaults({
      throttle: { enabled: false },
      retry: { enabled: false }
    })
  })
  probot.load(app)
})

test('receives /bounty command on issue comment', async function () {
  const mock = nock('https://api.github.com')
    .post(
      '/repos/Prom3th3us/bounty-icp-gh-app/issues/1/comments',
      (requestBody) => {
        assert.equal(requestBody, { body: 'Creating bounty of $100 for issue #1' })
        return true
      }
    )
    .reply(201, {})

  await probot.receive({
    name: 'issue_comment',
    id: '1',
    payload: {
      action: 'created',
      repository: {
        owner: {
          login: 'Prom3th3us'
        },
        name: 'bounty-icp-gh-app'
      },
      issue: {
        number: 1,
        pull_request: null
      },
      comment: {
        body: '/bounty 100'
      }
    }
  })

  assert.equal(mock.activeMocks(), [])
})

// Test for /attempt command on issue comment
test('receives /attempt command on issue comment', async function () {
  const mock = nock('https://api.github.com')
    .post(
      '/repos/Prom3th3us/bounty-icp-gh-app/issues/1/comments',
      (requestBody) => {
        assert.equal(requestBody, { body: 'Started working on attempt #1' })
        return true
      }
    )
    .reply(201, {})

  await probot.receive({
    name: 'issue_comment',
    id: '2',
    payload: {
      action: 'created',
      repository: {
        owner: {
          login: 'Prom3th3us'
        },
        name: 'bounty-icp-gh-app'
      },
      issue: {
        number: 1,
        pull_request: null
      },
      comment: {
        body: '/attempt #1'
      }
    }
  })

  assert.equal(mock.activeMocks(), [])
})

// Test for /approve command on PR comment
test('receives /approve command on PR comment', async function () {
  const mock = nock('https://api.github.com')
    .post(
      '/repos/Prom3th3us/bounty-icp-gh-app/issues/1/comments',
      (requestBody) => {
        assert.equal(requestBody, { body: 'Marking claim as ready to pay' })
        return true
      }
    )
    .reply(201, {})

  await probot.receive({
    name: 'pull_request_review_comment',
    id: '3',
    payload: {
      action: 'created',
      repository: {
        owner: {
          login: 'Prom3th3us'
        },
        name: 'bounty-icp-gh-app'
      },
      pull_request: {
        number: 1
      },
      comment: {
        body: '/approve'
      }
    }
  })

  assert.equal(mock.activeMocks(), [])
})

// Test for /claim command on PR body
test('receives /claim command on PR body', async function () {
  const mock = nock('https://api.github.com')
    .post(
      '/repos/Prom3th3us/bounty-icp-gh-app/issues/1/comments',
      (requestBody) => {
        assert.equal(requestBody, { body: 'Submitting claim for bounty of issue #1' })
        return true
      }
    )
    .reply(201, {})

  await probot.receive({
    name: 'pull_request',
    id: '4',
    payload: {
      action: 'opened',
      repository: {
        owner: {
          login: 'Prom3th3us'
        },
        name: 'bounty-icp-gh-app'
      },
      pull_request: {
        number: 1,
        body: '/claim #1'
      }
    }
  })

  assert.equal(mock.activeMocks(), [])
})

test.run()

// Allow using 't' as variable name for the tests
/* eslint-disable id-length */

const { test } = require('tape');

const User = require('../src/user');
const Member = require('../src/member');
const Admin = require('../src/admin');
const TrialUser = require('../src/trialUser');

test('Users should have a name', (t) => {
  const data = {
        name: 'john',
  }

  const user = new User(data);
  t.equal(user.name(), 'john');
  t.end();
});


test('Members should have an address', (t) => {
  const data = {
    name: 'john',
    address1: '452 Wilson Summit',
    address2: 'East Dawnshier, AK 96919',
  };
  const user = new User(data);
  const member = new Member(user);
  t.equal(member.address(), '452 Wilson Summit\nEast Dawnshier, AK 96919');
  t.end();
});

test('Admins should have a LDAP login', (t) => {
  const data = {
    name: 'john',
    enterpriseName: 'fooCorp',
  };
  const user = new User(data);
  const admin = new Admin(user);
  t.equal(admin.ldapLogin(), 'fooCorp/admin/john');
  t.end();
});

test('Trial users have a temporary name', (t) => {
  const data = {
    name: 'john',
    tempLogin: 'temp login',
  };
  const user = new TrialUser(data);
  t.equal(user.name(), 'temp login');
  t.end();
});

test('Compute numbers of days left for trial users', (t) => {
  const now = new Date().getTime();
  const twoDaysAgo = new Date(now - 1000 * 60 * 60 * 24 * 2);
  const data = {
    tempLogin: 'temp login',
    createdAt: twoDaysAgo,
  };
  const trialUser = new TrialUser(data);
  t.equal(trialUser.daysLeft(), 2);
  t.end();
});

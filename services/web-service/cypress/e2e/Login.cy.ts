describe('Login', () => {
  it('Correct credentials log in', () => {
    cy.visit('http://dottit.local/auth/login')

    cy.get('input[name="username"]').type('test')
    cy.get('input[name="password"]').type('Password123')

    cy.get('button[type="submit"]').click()

    cy.url({ timeout: 10000 }).should('contain', '/boards')
  })

  it('Incorrect username, failed login', () => {
    cy.visit('http://dottit.local/auth/login')

    cy.get('input[name="username"]').type('___')
    cy.get('input[name="password"]').type('Password123')

    cy.get('button[type="submit"]').click()

    cy.get('form > .MuiFormHelperText-root:last-of-type').should('have.text', 'failed to login.')
  })
})

export { }

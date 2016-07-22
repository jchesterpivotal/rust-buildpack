$: << 'cf_spec'
require 'spec_helper'

describe 'CF Go Buildpack' do
  subject(:app) { Machete.deploy_app(app_name) }

  let(:browser) { Machete::Browser.new(app) }

  after { Machete::CF::DeleteApp.new.execute(app) }

  context 'with uncached buildpack dependencies' do
    context "hello world server" do
      let(:app_name) { 'rust_app_hello_world_server' }

      it "boots up a rust server" do
        expect(app).to be_running
        expect(app).to have_logged('Listening on http://0.0.0.0:8080')

        browser.visit_path('/')
        expect(browser).to have_body('Hello World!')
      end
    end

    context "simple iron web app" do
      let(:app_name) { 'iron_app' }

      it "boots up an iron app" do
        expect(app).to be_running
        expect(app).to have_logged('Booting up Hello World Iron app at http://0.0.0.0:8080')

        browser.visit_path('/')
        expect(browser).to have_body('Hello World from the Iron web framework!')
      end
    end
  end
end

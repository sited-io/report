job "report" {
  datacenters = ["dc1"]
  type        = "service"

  group "report-api" {
    count = 2

    network {
      mode = "bridge"

      port "grpc" {}
    }

    service {
      name = "report-api"
      port = "grpc"

      connect {
        sidecar_service {}
      }

      check {
        type     = "grpc"
        interval = "20s"
        timeout  = "2s"
      }
    }

    task "report-api" {
      driver = "docker"

      vault {
        policies = ["service-report"]
      }

      template {
        destination = "${NOMAD_SECRETS_DIR}/.env"
        env         = true
        change_mode = "restart"
        data        = <<EOF
HOST='0.0.0.0:{{ env "NOMAD_PORT_grpc" }}'

{{ with secret "kv2/data/services/report" }}
GH_APP_ID='{{ .Data.data.GH_APP_ID }}'
GH_APP_PRIVATE_KEY='{{ .Data.data.GH_APP_PRIVATE_KEY }}'
{{ end }}

{{ with nomadVar "nomad/jobs/report" }}
RUST_LOG='{{ .LOG_LEVEL }}'
{{ end }}
EOF
      }

      config {
        image      = "__IMAGE__"
        force_pull = true
      }
    }
  }
}

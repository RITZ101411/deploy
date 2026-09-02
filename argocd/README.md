# ArgoCD Setup

GitOps でクラスタを管理する。`manifests/` の変更が自動でクラスタに反映される。

## 前提

- k3s が動いている
- CD ワークフローでイメージが ghcr.io に push されている

## 1. ArgoCD インストール

```bash
sudo kubectl create namespace argocd
sudo kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml

# 起動待ち
sudo kubectl wait --for=condition=available --timeout=300s deployment --all -n argocd
```

## 2. 初期パスワード取得

```bash
sudo kubectl -n argocd get secret argocd-initial-admin-secret \
  -o jsonpath="{.data.password}" | base64 -d; echo
```

ユーザー名は `admin`。

## 3. Application を適用

```bash
sudo kubectl apply -f argocd/application.yaml
```

これで ArgoCD が `manifests/` を監視し、自動同期を開始する。

## 4. 状態確認

```bash
sudo kubectl get applications -n argocd
sudo kubectl get pods -n deploy-system
```

## UI (任意)

port-forward でローカルからアクセス:

```bash
sudo kubectl port-forward svc/argocd-server -n argocd 8080:443
```

https://localhost:8080 で admin / (上記パスワード) でログイン。

## メモ

- `manifests/` を git に push すると ArgoCD が自動で反映する
- イメージ更新（新しい sha タグ）を反映するには、manifest の image タグ更新か Image Updater が必要（現状は `latest` タグ運用）

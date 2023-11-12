// Variáveis globais para os gráficos
var barChart;
var pieChart;
var isMessagePanelOpen = false;

document.addEventListener("DOMContentLoaded", function () {
    // Crie o gráfico de barras inicialmente
    createBarChart([0, 0, 0]);

    // Crie o gráfico de pizza inicialmente
    createPieChart([0, 0, 0]);

    // Atualize a dashboard a cada 5 segundos (simulação)
    setInterval(updateDashboard, 5000);

    // Inicialize a dashboard
    updateDashboard();

    // Adicione um evento de clique ao link "Paciente 1"
    const paciente1Link = document.querySelector('a[data-value="patient1"]');
    if (paciente1Link) {
        paciente1Link.addEventListener('click', function (event) {
            event.preventDefault();
            window.location.href = 'inicio.html?patient=patient1';
        });
    }

    // Adicione um evento de clique ao link "Paciente 2" (se necessário)
    const paciente2Link = document.querySelector('a[data-value="patient2"]');
    if (paciente2Link) {
        paciente2Link.addEventListener('click', function (event) {
            event.preventDefault();
            window.location.href = 'inicio.html?patient=patient2';
        });
    }
});

// Função para buscar dados do backend
async function fetchDataFromBackend() {
    try {
        const response = await fetch('http://localhost:8000/energizz');
        if (!response.ok) {
            throw new Error('Erro ao buscar dados do backend');
        }
        const data = await response.json();

        // Certifique-se de que a estrutura dos dados corresponde ao esperado
        if (Array.isArray(data) && data.length > 0) {
            return data[0]; // Vamos assumir que estamos usando apenas o primeiro registro para simplificar
        } else {
            throw new Error('Estrutura de dados do backend inválida');
        }
    } catch (error) {
        console.error('Erro na solicitação ao backend:', error);
        throw error; // ou manipule o erro conforme necessário
    }
}

// Função para criar o gráfico de barras
function createBarChart(data) {
    var ctx = document.getElementById('myChart').getContext('2d');
    barChart = new Chart(ctx, {
        type: 'bar',
        data: {
            labels: ['Temperatura', 'Umidade', 'Pressão'],
            datasets: [{
                label: 'Valores',
                data: data,
                backgroundColor: 'rgba(75, 192, 192, 0.2)',
                borderColor: 'rgba(75, 192, 192, 1)',
                borderWidth: 1
            }]
        },
        options: {
            responsive: false,
            maintainAspectRatio: false,
            width: 300,
            height: 100,
        }
    });
}

// Função para criar o gráfico de pizza
function createPieChart(data) {
    var ctx = document.getElementById('pieChart').getContext('2d');
    pieChart = new Chart(ctx, {
        type: 'pie',
        data: {
            labels: ['Temperatura', 'Umidade', 'Pressão'],
            datasets: [{
                label: 'Valores',
                data: data,
                backgroundColor: ['red', 'green', 'blue'],
                borderColor: ['red', 'green', 'blue'],
                borderWidth: 1
            }]
        },
        options: {
            responsive: false,
            maintainAspectRatio: false,
        }
    });
}

// Função para exibir alertas
//function showAlertMessage(message) {
    // Implemente a lógica para exibir alertas na interface do usuário
 //   console.log('Alerta:', message);
//}

function showAlertMessage(message) {
    const alertMessage = document.getElementById('alertMessage');
    const alertText = document.getElementById('alertText');

    alertText.textContent = message;
    alertMessage.classList.remove('hidden');
    isMessagePanelOpen = true;
}

function hideAlertMessage() {
    const alertMessage = document.getElementById('alertMessage');
    alertMessage.classList.add('hidden');
}

function closeMessagePanel() {
    const messagePanel = document.getElementById('alertMessage');
    messagePanel.classList.add('hidden');

    // Remova o conteúdo da mensagem
    const alertText = document.getElementById('alertText');
    alertText.textContent = '';
}


// Função para adicionar mensagens ao log
function addToMessageLog(message) {
    // Implemente a lógica para adicionar mensagens a um log na interface do usuário
    console.log('Log:', message);
}

function redirectToHome() {
    // Redireciona para a página inicial (substitua 'index.html' pelo caminho correto)
    window.location.href = 'index.html';
}

function redirectToLogin() {
    // Redireciona para a página de login (substitua 'login.html' pelo caminho correto)
    window.location.href = '/static/login.html';
}

function redirectToBatimentos() {
    // Redireciona para a página de batimentos (substitua 'batimentos.html' pelo caminho correto)
    window.location.href = '/static/batimentos.html';
}

// Função para atualizar a dashboard
async function updateDashboard() {
    try {
        const data = await fetchDataFromBackend();

        const { temperatura, umidade, pressao } = data;

        if (temperatura !== undefined && umidade !== undefined && pressao !== undefined) {
            // Atualiza a exibição da pressão cardíaca
            document.getElementById('heartRate').textContent = `${temperatura} °C`;
            document.getElementById('spo2').textContent = `${umidade}%`;
            document.getElementById('hPa').textContent = `${pressao} hPa`;

            // Atualize o gráfico de barras
            barChart.data.datasets[0].data = [temperatura, umidade, pressao];
            barChart.update();

            // Atualize o gráfico de pizza
            pieChart.data.datasets[0].data = [temperatura, umidade, pressao];
            pieChart.update();

            // Verifica a temperatura e exibe mensagens de alerta
            if (temperatura >= 37.5) {
                const message = "Sua temperatura está elevada. Recomendamos procurar atendimento médico.";
                showAlertMessage(message);
                addToMessageLog(message);
            }

            // Verifica a umidade e exibe mensagens de alerta
            if (umidade < 30) {
                const message = "A umidade está muito baixa. Considere usar um umidificador.";
                showAlertMessage(message);
                addToMessageLog(message);
            }

            // Verifica a pressão e exibe mensagens de alerta
            if (pressao > 140) {
                const message = "Sua pressão está alta. Recomendamos procurar atendimento médico.";
                showAlertMessage(message);
                addToMessageLog(message);
            }
        } else {
            console.error('Dados do backend incompletos ou inválidos');
        }
    } catch (error) {
        console.error('Erro ao atualizar a dashboard:', error);
        // Manipule o erro conforme necessário (exibindo uma mensagem para o usuário, etc.)
    }
}

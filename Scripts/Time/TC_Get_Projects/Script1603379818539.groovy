import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequestAndVerify(findTestObject('Time/GET_Projects', [('url') : GlobalVariable.url, ('token') : GlobalVariable.token]))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'data[0].projectId', "2")
WS.verifyElementPropertyValue(response, 'data[0].projectName', "Klan Uchiha")
WS.verifyElementPropertyValue(response, 'data[0].customerId', "1")
WS.verifyElementPropertyValue(response, 'data[0].customerName', "Sasuke Uchiha")
WS.verifyElementPropertyValue(response, 'data[0].isDeleted', "0")
WS.verifyElementPropertyValue(response, 'data[0].admins.employeeId', "3")
WS.verifyElementPropertyValue(response, 'data[0].admins.name', "Nia Middle Test")
WS.verifyElementPropertyValue(response, 'data[0].activities[0].id', "1")
WS.verifyElementPropertyValue(response, 'data[0].activities[0].name', "Requirement ke-1")
WS.verifyElementPropertyValue(response, 'data[0].activities[0].isDeleted', "0")
WS.verifyElementPropertyValue(response, 'data[0].activities[1].id', "3")
WS.verifyElementPropertyValue(response, 'data[0].activities[1].name', "test")
WS.verifyElementPropertyValue(response, 'data[0].activities[1].isDeleted', "0")

WS.verifyElementPropertyValue(response, 'data[5].projectId', "9")
WS.verifyElementPropertyValue(response, 'data[5].projectName', "nestle alokasi")
WS.verifyElementPropertyValue(response, 'data[5].customerId', "11")
WS.verifyElementPropertyValue(response, 'data[5].customerName', "PT Nestle")
WS.verifyElementPropertyValue(response, 'data[5].description', "test 456")
WS.verifyElementPropertyValue(response, 'data[5].isDeleted', "0")
WS.verifyElementPropertyValue(response, 'data[5].admins.employeeId', "4")
WS.verifyElementPropertyValue(response, 'data[5].admins.name', "Sherina Melodi Darmawan")
WS.verifyElementPropertyValue(response, 'data[5].activities[0].id', "9")
WS.verifyElementPropertyValue(response, 'data[5].activities[0].name', "activity 1")
WS.verifyElementPropertyValue(response, 'data[5].activities[0].isDeleted', "0")
